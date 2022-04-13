use std::{collections::HashMap, path::PathBuf};

pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub cxx: bool,
}

pub struct Compilers {
    pub c: String,
    pub cxx: String,
}

pub struct PackageCommon {
    pub name: String,
    pub meta: PackageMeta,
    pub sources: Vec<PathBuf>,
}

pub struct PackageBin {
    pub common: PackageCommon,
    pub defines: HashMap<String, String>,
    pub compilers: Compilers,
}

pub struct PackageLib {
    pub common: PackageCommon,
}
