use heraclitus_compiler::prelude::*;
use crate::{utils::{ParserMetadata, TranslateMetadata}, modules::types::{Type, Typed}};
use crate::translate::module::TranslateModule;
use super::{variable_name_extensions, handle_variable_reference};

#[derive(Debug, Clone)]
pub struct VariableGet {
    pub name: String,
    kind: Type,
    global_id: Option<usize>
}

impl Typed for VariableGet {
    fn get_type(&self) -> Type {
        self.kind.clone()
    }
}

impl SyntaxModule<ParserMetadata> for VariableGet {
    syntax_name!("Variable Access");

    fn new() -> Self {
        VariableGet {
            name: String::new(),
            kind: Type::Null,
            global_id: None
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let tok = meta.get_current_token();
        self.name = variable(meta, variable_name_extensions())?;
        (self.global_id, self.kind) = handle_variable_reference(meta, tok, &self.name)?;
        Ok(())
    }
}

impl TranslateModule for VariableGet {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        match self.global_id {
            Some(id) => format!("${{__{id}_{}}}", self.name),
            None => format!("${{{}}}", self.name)
        }
    }
}