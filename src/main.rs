use std::path::PathBuf;

use clap::Parser;
use handlebars::Handlebars;
use serde_json::Value;

/// An easy templating system where all you do is pass in a Handlebars template and a data file and you're good to go
#[derive(Debug, Parser)]
struct Cli {
    /// Path to a template file
    template: PathBuf,

    /// Path to a data file (supports JSON, YAML, and TOML)
    data: PathBuf,
}

fn main() {
    let Cli { template, data } = Cli::parse();

    let template_file =
        std::fs::read_to_string(&template).expect("Failed to read from template file");
    let data_file = std::fs::read_to_string(&data).expect("Failed to read from data file");

    let data: Value = match data
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default()
        .as_str()
    {
        "json" => serde_json::from_str(&data_file).expect("Failed to parse JSON"),
        "yaml" | "yml" => serde_yaml::from_str(&data_file).expect("Failed to parse YAML"),
        "toml" => toml::from_str(&data_file).expect("Failed to parse TOML"),
        ext => panic!("File extension {} not recognied", ext),
    };

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("template", &template_file)
        .expect("Failed to register template file with Handlebars");

    let rendered = handlebars
        .render("template", &data)
        .expect("Failed to render template");

    println!("{}", rendered);
}
