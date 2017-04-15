#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate regex;
extern crate clap;
#[macro_use]
extern crate nom;

mod template;
mod structures;
mod util;
mod cli;
mod parser;

use std::io::prelude::*;
use std::fs::File;

use structures::{Config, Root, Module, Validate};
use template::{class_template, function_template};
use util::{write_to_file, create_package};

fn read_toml(conf_file: &str) -> String {
    let file = File::open(conf_file);

    let mut file_content = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match file.read_to_string(&mut file_content) {
        Ok(x) => println!("Read size: {}", x),
        Err(error) => panic!("There was an error {:?} reading the config file", error),
    }

    // return the file content.
    file_content
}

fn validate (root: Root) -> Root {
    for package in &root.packages {
        let ref modules: Vec<Module> = package.modules;

        for module in modules {
            let ref functions = module.functions;
            let ref classes = module.classes;

            for function in functions {
                let is_valid: bool = function.validate_case();

                if !is_valid {
                    panic!("Invalid function name format");
                }
            }

            for class in classes {
                let is_valid: bool = class.validate_case();

                if !is_valid {
                    panic!("Invalid class name format");
                }
            }
        }
    }

    root
}

fn generate(skip_validations: bool, conf_file: String) {
    let toml_file_content = read_toml(&conf_file);
    let config: Config = toml::from_str(&toml_file_content).unwrap();

    // Root have packages
    // Packages have modules
    // Modules have functions
    let mut root = config.root;

    if !skip_validations {
        root = validate(root);
    }

    for package in root.packages {
        create_package(&package.name);

        let path = package.name;

        let modules = package.modules;

        for module in modules {
            let functions = module.functions;
            let ref filename = module.name;

            let classes = module.classes;
            let mut content = String::new();

            for class in classes {
                content += &class_template(class);
                write_to_file(&path, &filename, &content);
            }

            for function in functions {
                content += &function_template(function);
            }

            write_to_file(&path, &filename, &content);
        }
    }
}


fn main() {
    let src = r#"
class Animal:
    def howdie(self):
        pass

def hello():
    print "This is the hello function"
"#;
    // Call cli main function
    let cli_values = cli::main();
    let skip_validations = cli_values.skip_validations;
    let conf_file = cli_values.conf_file.unwrap();
    let parse = cli_values.parse;

    if parse {
        parser::parse(src.to_string());
    } else {
        generate(skip_validations, conf_file);
    }
}
