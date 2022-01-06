use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, Error, Write};

#[derive(Debug, Deserialize)]
struct PokemonTypeSerialized {
    name: String,
    effective: Vec<String>,
    not_effective: Vec<String>,
    no_effect: Vec<String>,
}

#[derive(Debug)]
struct PokemonType<'a> {
    name: &'a str,
    attack_effective: Vec<&'a str>,
    attack_not_effective: Vec<&'a str>,
    attack_no_effect: Vec<&'a str>,
    defense_effective: Vec<&'a str>,
    defense_not_effective: Vec<&'a str>,
    defense_no_effect: Vec<&'a str>,
}

struct PokemonTypeNameRegistry {
    registry: HashSet<String>,
}

impl PokemonTypeNameRegistry {
    fn reference(&self, pokemon_type: &String) -> &str {
        self.registry.get(pokemon_type).unwrap()
    }
}

fn main() {
    let serialized_types: Vec<PokemonTypeSerialized> =
        serde_yaml::from_reader(BufReader::new(File::open("data.yaml").unwrap())).unwrap();

    // To avoid millions of strings with the same value, we register them here once and only reference them from now on
    let registry = PokemonTypeNameRegistry {
        registry: serialized_types
            .iter()
            .map(|t| &t.name)
            .cloned()
            .collect::<HashSet<String>>(),
    };

    let mut types = Vec::with_capacity(serialized_types.len());
    for t in serialized_types.iter() {
        let name = registry.reference(&t.name);

        let attack_effective = t.effective.iter().map(|n| registry.reference(n)).collect();

        let attack_not_effective = t
            .not_effective
            .iter()
            .map(|n| registry.reference(n))
            .collect();

        let attack_no_effect = t.no_effect.iter().map(|n| registry.reference(n)).collect();

        let mut defense_effective = Vec::new();
        let mut defense_not_effective = Vec::new();
        let mut defense_no_effect = Vec::new();

        for tt in serialized_types.iter() {
            if tt.effective.iter().any(|n| n == name) {
                defense_effective.push(registry.reference(&tt.name));
            }
            if tt.not_effective.iter().any(|n| n == name) {
                defense_not_effective.push(registry.reference(&tt.name))
            }
            if tt.no_effect.iter().any(|n| n == name) {
                defense_no_effect.push(registry.reference(&tt.name))
            }
        }

        types.push(PokemonType {
            name,
            attack_effective,
            attack_not_effective,
            attack_no_effect,
            defense_effective,
            defense_not_effective,
            defense_no_effect,
        });
    }

    let selected_type = ask("Select a type: ").unwrap();
    for t in types.iter() {
        if t.name == selected_type {
            println!("{:#?}", t);
        }
    }
}

fn ask(question: &'static str) -> Result<String, Error> {
    println!("{}", question);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}
