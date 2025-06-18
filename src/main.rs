use std::{fs, path::PathBuf};

use clap::Parser;
use handlebars::Handlebars;
use serde_json::Value;

const TEMPLATE: &str = "template";

/// An easy templating system where all you do is pass in a Handlebars template and a JSON/YAML/TOML data file and you're good to go
#[derive(Debug, Parser)]
struct Cli {
    /// Path to a template file
    template_file: PathBuf,

    /// Path to a data file (supports JSON, YAML, and TOML)
    data_file: PathBuf,
}

fn main() {
    let Cli {
        template_file,
        data_file,
    } = Cli::parse();

    let data_file_string = fs::read_to_string(&data_file).expect("Failed to read from data file");

    let data: Value = match data_file
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default()
        .as_str()
    {
        "json" => serde_json::from_str(&data_file_string).expect("Failed to parse JSON"),
        "yaml" | "yml" => serde_yaml::from_str(&data_file_string).expect("Failed to parse YAML"),
        "toml" => toml::from_str(&data_file_string).expect("Failed to parse TOML"),
        ext => panic!("File extension {ext} for data file at {data_file:?} not recognized"),
    };

    let template_file_string =
        fs::read_to_string(&template_file).expect("Failed to read from template file");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(TEMPLATE, &template_file_string)
        .expect("Failed to register template file with Handlebars");

    let rendered = handlebars
        .render(TEMPLATE, &data)
        .expect("Failed to render template");

    println!("{}", rendered);
}
