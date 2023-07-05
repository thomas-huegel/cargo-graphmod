use crate::use_cases::storage_trait::Storage;

pub struct PostgresDb {}

impl Storage for PostgresDb {
    fn store(&self, _domain: &crate::domain::Domain) {
    }
}