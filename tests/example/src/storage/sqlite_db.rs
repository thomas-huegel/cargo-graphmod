use crate::use_cases::storage_trait::Storage;

pub struct SQLiteDb {
}

impl Storage for SQLiteDb {
    fn store(&self, _domain: &crate::domain::Domain) {
    }
}