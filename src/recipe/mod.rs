mod c;
pub mod rust;

use crate::config::{Contract, TemplateType};
use crate::project_context::{BuildConfig, Context};
use crate::signal::Signal;
use anyhow::Result;

pub fn get_recipe(context: Context, template_type: TemplateType) -> Result<Box<dyn Recipe>> {
    match template_type {
        TemplateType::Rust => Ok(Box::new(rust::Rust::new(context))),
        TemplateType::C => Ok(Box::new(c::C::<c::CBin>::new(context))),
        TemplateType::CSharedLib => Ok(Box::new(c::C::<c::CSharedLib>::new(context))),
    }
}

pub trait Recipe {
    fn exists(&self, contract: &Contract) -> bool;
    fn create_contract(
        &self,
        contract: &Contract,
        rewrite_config: bool,
        signal: &Signal,
    ) -> Result<()>;
    fn run(&self, contract: &Contract, build_cmd: String, signal: &Signal) -> Result<()>;
    fn run_build(&self, contract: &Contract, config: BuildConfig, signal: &Signal) -> Result<()>;
    fn clean(&self, contracts: &[Contract], signal: &Signal) -> Result<()>;
}
