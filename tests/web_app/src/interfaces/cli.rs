use crate::use_cases::UseCases as UC;

pub struct CliApp {
    use_cases: UC,
}

impl CliApp {
    pub fn new(use_cases: UC) -> Self {
        Self { use_cases }
    }
}