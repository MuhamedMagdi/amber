use heraclitus_compiler::prelude::*;
use crate::{modules::expression::expr::Expr, translate::module::TranslateModule};
use crate::utils::{ParserMetadata, TranslateMetadata};
use super::{variable_name_extensions, handle_variable_reference};

#[derive(Debug, Clone)]
pub struct VariableSet {
    name: String,
    value: Box<Expr>,
    global_id: Option<usize>
}

impl SyntaxModule<ParserMetadata> for VariableSet {
    syntax_name!("Variable Set");

    fn new() -> Self {
        VariableSet {
            name: String::new(),
            value: Box::new(Expr::new()),
            global_id: None
        }
    }
    
    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let tok = meta.get_current_token();
        self.name = variable(meta, variable_name_extensions())?;
        token(meta, "=")?;
        syntax(meta, &mut *self.value)?;
        (self.global_id, _) = handle_variable_reference(meta, tok, &self.name)?;
        Ok(())
    }
}

impl TranslateModule for VariableSet {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let name = self.name.clone();
        let expr = self.value.translate(meta);
        match self.global_id {
            Some(id) => format!("__{id}_{name}={expr}"),
            None => format!("{name}={expr}")
        }
    }
}