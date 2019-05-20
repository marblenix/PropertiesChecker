#[macro_use]
extern crate clap;
use clap::App;
use properties::Property;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct File<'a> {
    name: String,
    properties: Vec<Property<'a>>,
}

impl<'a> File<'a> {
    fn new() -> Self {
        File {
            name: "".to_string(),
            properties: vec![],
        }
    }


    fn contains_key(self, key: &str) -> bool {
        for property in self.properties {
            if property.key() == key {
                return true;
            }
        }
        return false;
    }
}

/// WARNING: leaks
///
/// use with care
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn file_to_list(file: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(file)?;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => eprintln!("Failed reading lines from file: {}", e),
        }
    }
    Ok(lines)
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let separator: char = matches.values_of("separator").unwrap().last().unwrap().parse().unwrap();
    let mut files: Vec<File> = Vec::new();
    for filename in matches.values_of("FILES").unwrap() {
        let file_lines = file_to_list(filename);
        match file_lines {
            Ok(lines) => {
                let mut file = File::new();
                file.name = filename.to_string();
                for line in lines {
                    if line.trim().is_empty() {
                        continue;
                    }
                    file.properties.push(properties::split(
                        string_to_static_str(line),
                        Some(separator)
                    ));
                }
                files.push(file);
            }
            Err(e) => eprintln!("Failed to read file: {}", e),
        }
    }

    for file in dbg!(files) {
        dbg!(file.name);
    }
}
