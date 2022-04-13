use std::{any::Any, panic::catch_unwind};

use super::package::{PackageBin, PackageLib};

pub struct Compiler {
    opt: u32,
    debug: bool,
}

impl Compiler {
    pub fn new(opt: u32, debug: bool) -> Self {
        Self { opt, debug }
    }
    pub fn compile_bin(&self, bin: &PackageBin) -> Result<(), Box<dyn Any + Send + 'static>> {
        catch_unwind(move || {
            let builder = &mut cc::Build::new();
            builder.debug(self.debug);
            builder.opt_level(self.opt);
            builder.cpp(bin.common.meta.cxx);
            builder.compiler(if bin.common.meta.cxx {
                &bin.compilers.cxx
            } else {
                &bin.compilers.c
            });
            builder.files(&bin.common.sources);
            for (key, value) in &bin.defines {
                builder.define(key.as_str(), value.as_str());
            }
            builder.compile(bin.common.meta.name.as_str());
        })?;
        Ok(())
    }
    pub fn compile_lib(&self, lib: &PackageLib) -> Result<(), String> {
        todo!()
    }
}
