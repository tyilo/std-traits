import re
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import IO, Any
from tempfile import NamedTemporaryFile
import json


@dataclass
class FunctionSpec:
    must_use_reason: str | None
    unsafe: bool
    name: str
    definition: str
    call: str


def stringify_type(d: dict[str, Any]) -> str:
    [(typ, inner)] = d.items()
    match typ:
        case "primitive":
            return inner
        case "generic":
            return inner
        case "slice":
            return f"[{stringify_type(inner)}]"
        case "array":
            return f"[{stringify_type(inner['type'])}; {inner['len']}]"
        case "borrowed_ref":
            assert inner["lifetime"] is None
            s = "&"
            if inner["is_mutable"]:
                s += "mut "
            return s + stringify_type(inner["type"])
        case "tuple":
            return f"({', '.join(stringify_type(v) for v in inner)})"
        case "resolved_path":
            s = inner["path"]
            if s == "crate::cmp::Ordering":
                return "Ordering"
            args = inner["args"]
            if args:
                [(arg_typ, arg_inner)] = args.items()
                assert not arg_inner["constraints"]
                assert arg_typ == "angle_bracketed"
                s += (
                    "<"
                    + ", ".join(stringify_generic(t) for t in arg_inner["args"])
                    + ">"
                )

            return s
        case _:
            assert False, (typ, inner)


def stringify_generic(d: dict[str, Any]) -> str:
    [(typ, inner)] = d.items()
    match typ:
        case "type":
            return stringify_type(inner)
        case _:
            assert False, (typ, inner)


def stringify_arg(name: str, typ_s: str) -> str:
    if name == "self":
        match typ_s:
            case "Self":
                return "self"
            case "&Self":
                return "&self"
            case _:
                assert False, typ_s
    return f"{name}: {typ_s}"


def parse_fn(d: dict[str, Any]) -> FunctionSpec | None:
    inner = d["inner"]
    [(typ, spec)] = inner.items()
    if typ != "function":
        return None

    generics = spec["generics"]
    if generics["params"] or generics["where_predicates"]:
        return None

    if d["deprecation"] is not None:
        return None

    must_use_reason = None
    for attr in d["attrs"]:
        [(typ, inner)] = attr.items()
        match typ:
            case "must_use":
                [(reason_key, reason)] = inner.items()
                assert reason_key == "reason"
                assert must_use_reason is None
                must_use_reason = reason
            case "other":
                if inner.startswith("#[attr = Stability"):
                    if "Unstable" in inner or "since: Current" in inner:
                        return None
            case _:
                assert False, (typ, inner)

    name = d["name"]
    header = spec["header"]
    is_unsafe = header["is_unsafe"]

    sig = spec["sig"]

    args = []
    arg_names = []
    for arg_name, typ in sig["inputs"]:
        typ_s = stringify_type(typ)
        arg_names.append(arg_name)
        args.append(stringify_arg(arg_name, typ_s))

    definition = ""
    if is_unsafe:
        definition += "unsafe "

    definition += f"fn {name}("
    definition += ", ".join(args)
    definition += ")"
    definition += f" -> {stringify_type(sig['output'])}"

    call = f"{name}({", ".join(arg_names)})"

    return FunctionSpec(
        must_use_reason=must_use_reason,
        unsafe=is_unsafe,
        name=name,
        definition=definition,
        call=call,
    )


def parse_specs(crate: str, typ: str) -> list[FunctionSpec]:
    print(f"Processing {crate}::{typ}")

    path = (
        Path.home()
        / f".rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/{crate}.json"
    )
    with path.open("rb") as f:
        data = json.load(f)

    res = []
    for d in data["index"].values():
        impl = d["inner"].get("impl", {})

        if impl.get("for", {}).get("primitive") != typ:
            continue

        if impl["trait"] is not None:
            continue

        for item_id in impl["items"]:
            item_d = data["index"][str(item_id)]
            if fn := parse_fn(item_d):
                res.append(fn)

    return res


@dataclass
class Trait:
    example_implementor: str
    core_fns: list[FunctionSpec]
    std_fns: list[FunctionSpec]
    ignores: set[str]
    replacements: dict[str, str]


i32_core = parse_specs("core", "i32")
u32_core = parse_specs("core", "u32")

i32_fns = {s.name for s in i32_core}
u32_fns = {s.name for s in u32_core}

int_core = [s for s in i32_core if s.name in u32_fns]
signed_core = [s for s in i32_core if s.name not in u32_fns]
unsigned_core = [s for s in u32_core if s.name not in i32_fns]


TRAITS = {
    "FLOAT": Trait(
        example_implementor="f32",
        core_fns=parse_specs("core", "f32"),
        std_fns=parse_specs("std", "f32"),
        ignores={
            # Implemented on Number trait
            "to_be_bytes",
            "to_le_bytes",
            "to_ne_bytes",
            "from_be_bytes",
            "from_le_bytes",
            "from_ne_bytes",
            "abs",
            "signum",
            "div_euclid",
            "rem_euclid",
        },
        replacements={
            "u32": "Self::Bits",
            "[u8; 4]": "Self::Bytes",
        },
    ),
    "INTEGER": Trait(
        example_implementor="i32",
        core_fns=int_core,
        std_fns=[],
        ignores={
            # Implemented on Number trait
            "to_be_bytes",
            "to_le_bytes",
            "to_ne_bytes",
            "from_be_bytes",
            "from_le_bytes",
            "from_ne_bytes",
            "abs",
            "signum",
            "div_euclid",
            "rem_euclid",
        },
        replacements={
            "abs_diff(self, other: Self) -> u32": "abs_diff(self, other: Self) -> Self::Unsigned",
        },
    ),
    "SIGNED": Trait(
        example_implementor="i32",
        core_fns=signed_core,
        std_fns=[],
        ignores=set(),
        replacements={
            "_unsigned(self) -> u32": "_unsigned(self) -> Self::Unsigned",
            "_unsigned(self, rhs: u32)": "_unsigned(self, rhs: Self::Unsigned)",
            "unsigned_abs(self) -> u32": "unsigned_abs(self) -> Self::Unsigned",
        },
    ),
    "UNSIGNED": Trait(
        example_implementor="u32",
        core_fns=unsigned_core,
        std_fns=[],
        ignores=set(),
        replacements={
            "_signed(self, rhs: i32)": "_signed(self, rhs: Self::Signed)",
            "_signed(self) -> i32": "_signed(self) -> Self::Signed",
            "_signed_diff(self, rhs: Self) -> Option<i32>": "_signed_diff(self, rhs: Self) -> Option<Self::Signed>",
        },
    ),
}


def print_decl(dst: IO[str], indent: str, trait: Trait, impl: bool) -> None:
    print(f"{indent}// Generated by generate_delegates.py", file=dst)
    print(file=dst)

    core_fns = {s.name for s in trait.core_fns}
    std_fns = [s for s in trait.std_fns if s.name not in core_fns]

    for is_std, fns in enumerate([trait.core_fns, std_fns]):
        if is_std:
            cfg = f'{indent}#[cfg(feature = "std")]\n'
        else:
            cfg = ""
        for fn in fns:
            if fn.name in trait.ignores:
                continue

            definition = fn.definition.replace(trait.example_implementor, "Self")
            call = fn.call.replace(trait.example_implementor, "Self")
            for k, v in trait.replacements.items():
                definition = definition.replace(k, v)
                call = call.replace(k, v)

            if impl:
                print(
                    f"""{cfg}{indent}{definition} {{
{indent}    Self::{call}
{indent}}}
""",
                    file=dst,
                )
            else:
                ref = f"[`{trait.example_implementor}::{fn.name}`]"
                docs = f"{indent}/// See {ref}.\n"
                if fn.unsafe:
                    docs += f"""{indent}///
{indent}/// # Safety
{indent}///
{indent}/// See {ref}.
"""
                must_use = ""
                if fn.must_use_reason is not None:
                    must_use = f'{indent}#[must_use = "{fn.must_use_reason}"]\n'

                print(
                    f"""{docs}{cfg}{must_use}{indent}{definition};
""",
                    file=dst,
                )


START_RE = re.compile(r"^(?P<indent>\s*)// @START@ (?P<type>\S+) (?P<name>\S+)")

with NamedTemporaryFile("w") as dst:
    with open("src/num.rs", "r") as src:
        copy_lines = True
        for l in src:
            if copy_lines:
                print(l, end="", file=dst)
            if "@START@" in l:
                m = START_RE.search(l)
                assert m is not None
                group = m.groupdict()
                indent = group["indent"]
                trait = TRAITS[group["name"]]
                impl = group["type"] == "IMPL"
                print_decl(dst, indent, trait, impl)
                copy_lines = False

            if "@END@" in l:
                print(l, end="", file=dst)
                copy_lines = True

    dst.flush()
    shutil.move(dst.name, "src/num.rs")
