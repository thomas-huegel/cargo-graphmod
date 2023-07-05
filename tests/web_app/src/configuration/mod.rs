pub enum Interface {
    Cli,
    Web,
}

pub enum Storage {
    Postgres,
    SQLite,
}

pub struct Configuration {
    interface: Interface,
    storage: Storage,
}

impl Configuration {
    pub fn new(interface: Interface, storage: Storage) -> Self {
        Self {
            interface,
            storage,
        }
    }
}