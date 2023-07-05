use crate::use_cases::UseCases;

pub struct CliApp {
    use_cases: UseCases
}

impl CliApp {
    pub fn new(use_cases: UseCases) -> Self {
        Self { use_cases }
    }
}