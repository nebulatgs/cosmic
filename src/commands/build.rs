use std::{any::Any, panic::catch_unwind};

use crate::builder::compiler::Compiler;

pub fn build(opt: u32, debug: bool) -> Result<(), Box<dyn Any + Send + 'static>> {
    let compiler = Compiler::new(opt, debug);

    Ok(())
}
