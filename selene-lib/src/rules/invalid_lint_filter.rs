use super::*;
use std::convert::Infallible;

// This is a shell lint, meaning it does not have any behavior on its own
// The actual application of this lint is handled in lint_filtering.rs
// This exists for the purpose of letting the user disable lint related to lint filtering (such as invalid names)
pub struct InvalidLintFilterLint;

impl Rule for InvalidLintFilterLint {
    type Config = ();
    type Error = Infallible;

    fn new(_: Self::Config) -> Result<Self, Self::Error> {
        Ok(InvalidLintFilterLint)
    }

    fn pass(&self, _: &full_moon::ast::Ast, _: &Context, _: &AstContext) -> Vec<Diagnostic> {
        Vec::new()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn rule_type(&self) -> RuleType {
        RuleType::Correctness
    }
}
