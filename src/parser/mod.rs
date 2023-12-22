use crate::dependencies::DependencyPath;

pub mod rust_parser;

pub trait Parser {
    fn parse_dependencies(file_contents: &str) -> Vec<DependencyPath>;
}