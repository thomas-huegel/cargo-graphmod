use crate::domain::Domain;

pub trait Storage {
    fn store(&self, domain: &Domain);
}