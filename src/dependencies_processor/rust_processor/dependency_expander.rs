use std::collections::VecDeque;

use crate::dependencies::DependencyPath;

use super::{Dependency, DependencyKind, MOD};

const CRATE: &str = "crate";
const SELF: &str = "self";
const SUPER: &str = "super";

pub fn expand_dependency(
    dependency_components: &[String],
    pkg_name: &str,
    mut source_file_path: Vec<String>,
) -> Dependency {
    let fst = dependency_components
        .first()
        .expect("A dependency should not be empty!");
    if let Some(last) = source_file_path.last() {
        if last == MOD {
            source_file_path.pop();
        }
    }
    if fst == pkg_name || fst == CRATE {
        Dependency {
            path: DependencyPath(
                dependency_components
                    .iter()
                    .skip(1)
                    .map(|s| s.into())
                    .collect::<Vec<_>>(),
            ),
            kind: DependencyKind::Relative,
        }
    } else if fst == SUPER {
        let mut deps: VecDeque<_> = dependency_components.to_owned().into();
        while let Some(fst) = deps.front() {
            if fst == SUPER {
                deps.pop_front();
                source_file_path.pop();
            } else {
                break;
            }
        }
        source_file_path.append(&mut deps.iter().map(|s| s.into()).collect::<Vec<_>>());
        Dependency {
            path: DependencyPath(source_file_path),
            kind: DependencyKind::Relative,
        }
    } else if fst == SELF {
        source_file_path.append(
            &mut dependency_components
                .iter()
                .skip(1)
                .map(|s| s.into())
                .collect::<Vec<_>>(),
        );
        Dependency {
            path: DependencyPath(source_file_path),
            kind: DependencyKind::Relative,
        }
    } else {
        let source_file_path_len = source_file_path.len();
        source_file_path.append(
            &mut dependency_components
                .iter()
                .map(|s| s.into())
                .collect::<Vec<_>>(),
        );
        Dependency {
            path: DependencyPath(source_file_path),
            kind: DependencyKind::Ambiguous(source_file_path_len),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dependencies::DependencyPath,
        dependencies_processor::rust_processor::{
            dependency_expander::expand_dependency, Dependency, DependencyKind,
        },
    };

    #[test]
    fn it_belongs_to_my_crate() {
        let dependency = vec![
            String::from("my_crate"),
            String::from("foo"),
            String::from("bar"),
        ];
        let result = expand_dependency(&dependency, "my_crate", vec![]);
        assert_eq!(
            result,
            Dependency {
                path: DependencyPath(vec![String::from("foo"), String::from("bar")]),
                kind: DependencyKind::Relative,
            }
        );
    }

    #[test]
    fn it_belongs_to_crate() {
        let dependency = vec![
            String::from("crate"),
            String::from("foo"),
            String::from("bar"),
        ];
        let result = expand_dependency(&dependency, "my_crate", vec![]);
        assert_eq!(
            result,
            Dependency {
                path: DependencyPath(vec![String::from("foo"), String::from("bar")]),
                kind: DependencyKind::Relative,
            }
        );
    }

    #[test]
    fn it_belongs_to_a_supermodule() {
        let dependency = vec![
            String::from("super"),
            String::from("super"),
            String::from("foo"),
            String::from("bar"),
        ];
        let result = expand_dependency(
            &dependency,
            "my_crate",
            vec![
                String::from("aaa"),
                String::from("bbb"),
                String::from("ccc"),
                String::from("mod"),
            ],
        );
        assert_eq!(
            result,
            Dependency {
                path: DependencyPath(vec![
                    String::from("aaa"),
                    String::from("foo"),
                    String::from("bar")
                ]),
                kind: DependencyKind::Relative,
            }
        );
    }

    #[test]
    fn it_belongs_to_a_submodule_via_self() {
        let dependency = vec![
            String::from("self"),
            String::from("foo"),
            String::from("bar"),
        ];
        let result = expand_dependency(
            &dependency,
            "my_crate",
            vec![String::from("path"), String::from("mod")],
        );
        assert_eq!(
            result,
            Dependency {
                path: DependencyPath(vec![
                    String::from("path"),
                    String::from("foo"),
                    String::from("bar")
                ]),
                kind: DependencyKind::Relative,
            }
        );
    }

    #[test]
    fn it_may_belong_to_some_external_crate() {
        let dependency = vec![String::from("foo"), String::from("bar")];
        let result = expand_dependency(
            &dependency,
            "my_crate",
            vec![String::from("path"), String::from("mod")],
        );
        assert_eq!(
            result,
            Dependency {
                path: DependencyPath(vec![
                    String::from("path"),
                    String::from("foo"),
                    String::from("bar")
                ],),
                kind: DependencyKind::Ambiguous(1)
            }
        );
    }
}
