import re
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import IO
from tempfile import NamedTemporaryFile


@dataclass
class FunctionSpec:
    unsafe: bool
    name: str
    definition: str
    call: str


ARG_TYPE_RE = re.compile(r": [^,)]+")


def parse_specs(filename: str) -> list[FunctionSpec]:
    res = []
    with open(filename) as f:
        is_unstable = False
        for l in f:
            l = l.strip()
            if l == "source":
                is_unstable = True
                continue
            if l.startswith("impl"):
                is_unstable = False
                continue
            if not l.startswith("pub "):
                continue
            if " fn " not in l:
                continue

            if is_unstable:
                is_unstable = False
                continue

            l = l.removeprefix("pub ")
            l = l.removeprefix("const ")
            unsafe = "unsafe fn " in l
            call = l.split(" -> ")[0].removeprefix("unsafe ").removeprefix("fn ")
            call = ARG_TYPE_RE.sub("", call)
            name = call.split("(")[0]

            res.append(
                FunctionSpec(
                    unsafe=unsafe,
                    name=name,
                    definition=l,
                    call=call,
                )
            )

    return res


@dataclass
class Trait:
    example_implementor: str
    core_fns: list[FunctionSpec]
    std_fns: list[FunctionSpec]
    ignores: set[str]
    replacements: dict[str, str]


i32_core = parse_specs("spec/i32_core.txt")
u32_core = parse_specs("spec/u32_core.txt")

i32_fns = {s.name for s in i32_core}
u32_fns = {s.name for s in u32_core}

int_core = [s for s in i32_core if s.name in u32_fns]
signed_core = [s for s in i32_core if s.name not in u32_fns]
unsigned_core = [s for s in u32_core if s.name not in i32_fns]


TRAITS = {
    "FLOAT": Trait(
        example_implementor="f32",
        core_fns=parse_specs("spec/f32_core.txt"),
        std_fns=parse_specs("spec/f32_std.txt"),
        ignores={
            # Implemented on Number trait
            "to_be_bytes",
            "to_le_bytes",
            "to_ne_bytes",
            "from_be_bytes",
            "from_le_bytes",
            "from_ne_bytes",
            # Has unstable trait bound
            "to_int_unchecked<Int>",
            # Deprecated
            "abs_sub",
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
            # Deprecated
            "min_value",
            "max_value",
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
        },
    ),
}


def print_decl(dst: IO[str], indent: str, trait: Trait, impl: bool) -> None:
    print(f"{indent}// Generated by generate_delegates.py", file=dst)
    print(file=dst)
    for is_std, fns in enumerate([trait.core_fns, trait.std_fns]):
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

            ref = f"[`{trait.example_implementor}::{fn.name}`]"
            docs = f"{indent}/// See {ref}.\n"
            if fn.unsafe:
                docs += f"""{indent}///
{indent}/// # Safety
{indent}///
{indent}/// See {ref}.
"""

            if impl:
                print(
                    f"""{cfg}{indent}{definition} {{
{indent}    Self::{call}
{indent}}}
""",
                    file=dst,
                )
            else:
                print(
                    f"""{docs}{cfg}{indent}{definition};
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
