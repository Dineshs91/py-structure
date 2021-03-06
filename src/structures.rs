extern crate serde;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
	pub root: Root,
}

// project root
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Root {
	pub name: String,

	#[serde(default)]
	pub packages: Vec<Package>,

	#[serde(default)]
	pub modules: Vec<Module>,
}

// python package. Any directory which has a __init__.py file.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
	pub name: String,

    #[serde(default)]
    pub packages: Vec<Package>,

	#[serde(default)]
	pub modules: Vec<Module>,
}

// python module, any python file.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
	pub name: String,
    pub description: Option<String>,

	#[serde(default)]
	pub functions: Vec<Function>,

	#[serde(default)]
	pub classes: Vec<Class>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Class {
	pub name: String,
	pub description: Option<String>,

    #[serde(default)]
    pub parents: Vec<String>,

	#[serde(default)]
	pub methods: Vec<Function>,
}

impl Validate for Class {
    fn validate_case(&self) -> bool {
        // Class name will be camel case.
        let re = Regex::new(r"^[A-Z]{1}[a-z]+$").unwrap();
        re.is_match(&self.name)
    }
}

// structure for a forming python function.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,

	#[serde(default)]
	pub parameters: Vec<String>,
}

impl Validate for Function {
    fn validate_case(&self) -> bool {
        // function name will be snake case.
        let re = Regex::new(r"^[a-z_]+$").unwrap();
        re.is_match(&self.name)
    }
}

pub trait Validate {
    fn validate_case(&self) -> bool;
}
