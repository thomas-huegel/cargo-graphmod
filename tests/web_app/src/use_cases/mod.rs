pub mod storage_trait;

use crate::domain::Domain;

use self::storage_trait::Storage;

pub struct UseCases {
    domain: Domain,
    storage: Box<dyn Storage>,
}

impl UseCases {
    pub fn new(domain: Domain, storage: Box<dyn Storage>) -> Self {
        Self {
            domain,
            storage,
        }
    }
}