use exhaustive_map::{ExhaustiveMap, Finite};
use rustdoc_types::{Attribute, Item, ItemEnum, Type};

#[derive(Debug, Finite)]
enum Stability {
    Stable,
    Unstable,
}

fn parse_stability_str(s: &str) -> Option<Stability> {
    let s = s.strip_prefix("#[attr = Stability ")?;
    let s = s.strip_prefix("{stability: Stability {level: ").unwrap();
    Some(match s.split_once(' ').unwrap().0 {
        "Stable" => Stability::Stable,
        "Unstable" => Stability::Unstable,
        _ => panic!(),
    })
}

fn calculate_stability(item: &Item) -> Option<Stability> {
    let mut stability = None;
    for attr in &item.attrs {
        if let Attribute::Other(s) = attr
            && let Some(stab) = parse_stability_str(s)
        {
            assert!(stability.is_none());
            stability = Some(stab);
        }
    }
    stability
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = std::env::home_dir().unwrap();
    path.push(".rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json");

    let json_string = std::fs::read_to_string(path)?;
    let krate: rustdoc_types::Crate = serde_json::from_str(&json_string)?;

    println!("the index has {} items", krate.index.len());

    let mut stable_counts: ExhaustiveMap<Option<Stability>, usize> = ExhaustiveMap::default();

    for (_id, item) in krate.index.iter() {
        stable_counts[calculate_stability(item)] += 1;

        let ItemEnum::Impl(imp) = &item.inner else {
            continue;
        };
        let Type::Primitive(typ) = &imp.for_ else {
            continue;
        };
        if typ != "u32" {
            continue;
        }
        if imp.trait_.is_some() {
            continue;
        }
        for fun_id in &imp.items {
            let fun_item = &krate.index[fun_id];
            let name = fun_item.name.as_ref().unwrap();
            let ItemEnum::Function(fun) = &fun_item.inner else {
                continue;
            };
            println!("{name}");
            if name == "bit_width" {
                println!(" {:#?}", fun);
            }
            println!(" {:#?}", fun_item.attrs);
            for attr in &fun_item.attrs {
                if let Attribute::Other(s) = attr
                    && let Some(stability) = parse_stability_str(s)
                {
                    match stability {
                        Stability::Stable => println!("  stable"),
                        Stability::Unstable => println!("  unstable"),
                    }
                }
            }
        }
    }

    println!("stability counts: {:#?}", stable_counts);

    Ok(())
}
