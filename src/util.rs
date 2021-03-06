use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use toml;
use serde_yaml;

use structures::{Config, Root};

const FILE_EXTENSION:&'static str = ".py";
const INIT_FILE:&'static str = "__init__.py";

pub fn read_file(filename: &str) -> String {
    let file = File::open(filename);

    let mut file_content = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(error) => panic!("There was an error {:?} reading the file {}", error, filename),
    }

    // return the file content.
    file_content
}

pub fn write_to_file(path: &Path, filename: &str, content: &str) {
	// Write the python source to file.
	// Args: file name, the content of the file.
	//
	// filename & content will be &str since we won't be manipulating it.
	let filename = filename.to_string() + FILE_EXTENSION;

	let path = path.join(filename);
	let mut file = match File::create(&path) {
		Err(e) => panic!("Error occurred while trying to create file {} {:?}", e, path),
		Ok(file) => file,
	};

	match file.write_all(content.as_bytes()) {
		Err(e) => println!("Error occurred while trying to write to file {}", e),
		Ok(_) => println!("Successfully written content to a file"),
	}
}

pub fn get_toml_result(root: Root) -> String {
    let config = Config {
        root: root
    };

    let toml_res = toml::Value::try_from(&config).unwrap();
    toml_res.to_string()
}

pub fn get_yaml_result(root: Root) -> String {
    let config = Config {
        root: root
    };

    let yaml_res = serde_yaml::to_string(&config).unwrap();
    yaml_res.to_string()
}

/// Write the parsed content to a config file. (Toml/Yaml).
pub fn write_to_config(conf_file: &str, toml_res: String) {
    let file = fs::File::create(conf_file);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => panic!("Error {} {}", e, conf_file)
    };

    file.write_all(toml_res.as_bytes()).expect("Could not write config to file");
}

pub fn create_package(package_path: &Path) {
	let init_file_path = INIT_FILE;

	match fs::create_dir_all(package_path) {
        Ok(_) => {},
        Err(e) => println!("Failed to create package {}", e)
    };

	let path = package_path.join(init_file_path);
	match File::create(&path) {
		Err(e) => panic!("Error occurred while trying to create file {}", e),
		Ok(file) => file,
	};
}