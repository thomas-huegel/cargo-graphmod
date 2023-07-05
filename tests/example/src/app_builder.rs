use crate::configuration::Configuration;
use crate::configuration::Storage::{Postgres, SQLite};
use crate::configuration::Interface::{Cli, Web};
use crate::domain::Domain;
use crate::interfaces::cli::CliApp;
use crate::interfaces::web::WebApp;
use crate::storage::postgres_db::PostgresDb;
use crate::storage::sqlite_db::SQLiteDb;
use crate::use_cases::{UseCases};

pub fn run_app() {
    let _configuration = Configuration::new(Web, Postgres);
    let _configuration2 = Configuration::new(Cli, SQLite);
    let storage1 = Box::new(PostgresDb{});
    let domain1 = Domain::new();
    let use_cases1 = UseCases::new(domain1, storage1);
    let storage2 = Box::new(SQLiteDb{});
    let domain2 = Domain::new();
    let use_cases2 = UseCases::new(domain2, storage2);
    let _web = WebApp::new(use_cases1);
    let _cli = CliApp::new(use_cases2);
}