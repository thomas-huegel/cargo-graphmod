use crate::dependencies::DependencyPath;

mod dependency_expander;
pub mod target_computer;

const MOD: &str = "mod";

#[derive(Debug, PartialEq)]
enum DependencyKind {
    Relative,
    Ambiguous(usize), // file path length (except "mod")
}

#[derive(Debug, PartialEq)]
pub struct Dependency {
    path: DependencyPath,
    kind: DependencyKind,
}