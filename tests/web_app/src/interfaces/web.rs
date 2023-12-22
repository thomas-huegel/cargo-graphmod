use crate::use_cases::UseCases;

pub struct WebApp {
    use_cases: UseCases,
}

impl WebApp {
    pub fn new(use_cases: UseCases) -> Self {
        Self { use_cases }
    }
}
