/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::collections::BTreeSet as Set;

use lazy_static::lazy_static;
use regex::Regex;

use crate::dependencies::DependencyPath;

use super::Parser;

const INPUT_SEPARATOR: &str = "::";

fn develop_innermost_dependencies(text: &str) -> Set<String> {
    lazy_static! {
        static ref PRODUCT: Regex = Regex::new(r"(?sm)(.*)\{(.*?)\}(.*)").unwrap();
    }
    let rewriting = PRODUCT
        .captures_iter(text)
        .flat_map(|cap| {
            cap[2]
                .split(',')
                .filter_map(|x| {
                    if x.trim().is_empty() {
                        None
                    } else {
                        Some(cap[1].to_string() + x + &cap[3])
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Set<String>>();
    if rewriting.is_empty() {
        Set::from([String::from(text)])
    } else {
        rewriting
    }
}

fn develop_all_dependencies(dependency: &str) -> Set<String> {
    let mut current_str = dependency.to_string();
    let mut current_deps = Set::from([dependency.to_string()]);
    while current_str.contains('{') {
        current_deps = current_deps
            .iter()
            .flat_map(|s| develop_innermost_dependencies(s.as_str()))
            .collect();
        current_str = current_deps
            .iter()
            .fold(String::new(), |acc, elem| acc + elem);
    }
    current_deps
}

fn keep_before_cfg_test(text: &str) -> Option<String> {
    lazy_static! {
        static ref KEEP_BEFORE: Regex = Regex::new(r"(?sm)(.*?)\#\[cfg\(test\)\].*").unwrap();
    }
    KEEP_BEFORE.captures(text).map(|cap| cap[1].to_string())
}

fn parse_use(text: &str) -> Vec<String> {
    lazy_static! {
        static ref USE: Regex = Regex::new(r"(?sm)^(?:\s)*(?:pub )?use (.*?);").unwrap();
    }
    USE.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn trim_spaces_and_as(dependency: &str) -> String {
    let mut vector = dependency.split_whitespace().collect::<Vec<_>>();
    let mut last_words = dependency.split_whitespace().rev();
    last_words.next();
    if let Some("as") = last_words.next() {
        vector.pop();
        vector.pop();
    }
    vector.join("")
}

pub struct RustParser {}

impl Parser for RustParser {
    fn parse_dependencies(file_contents: &str) -> Vec<DependencyPath> {
        let before_tests = match keep_before_cfg_test(file_contents) {
            None => file_contents.to_string(),
            Some(found) => found,
        };
        parse_use(&before_tests)
            .iter()
            .flat_map(|s| develop_all_dependencies(s))
            .map(|s| {
                s.split(INPUT_SEPARATOR)
                    .map(trim_spaces_and_as)
                    .collect::<Vec<String>>()
            })
            .map(DependencyPath)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet as Set;

    use crate::{
        dependencies::DependencyPath,
        parser::{
            rust_parser::{
                develop_all_dependencies, develop_innermost_dependencies, parse_use,
                trim_spaces_and_as, RustParser,
            },
            Parser,
        },
    };

    use super::keep_before_cfg_test;

    #[test]
    fn it_develops_innermost() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(
            result,
            Set::from([
                String::from("foo::{bar1, bar2, bar3:: boo}"),
                String::from("foo::{bar1, bar2, bar3::far}")
            ])
        );
    }

    #[test]
    fn it_develops_innermost_2() {
        let text = "crate::{foo::{bar}, baz, abc::def}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(
            result,
            Set::from([String::from("crate::{foo::bar, baz, abc::def}")])
        );
    }

    #[test]
    fn it_handles_newlines() {
        let text = "foo::{bar1, bar2, bar3::\n{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(
            result,
            Set::from([
                String::from("foo::{bar1, bar2, bar3::\nfar}"),
                String::from("foo::{bar1, bar2, bar3::\n boo}")
            ])
        );
    }

    #[test]
    fn it_skips_blanks() {
        let text = "foo::{bar1, bar2, bar3::{far, boo, }}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(
            result,
            Set::from([
                String::from("foo::{bar1, bar2, bar3:: boo}"),
                String::from("foo::{bar1, bar2, bar3::far}")
            ])
        );
    }

    #[test]
    fn it_does_nothing() {
        let text = "foo::bar";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, Set::from([String::from("foo::bar")]));
    }

    #[test]
    fn it_trims_spaces_and_as() {
        let text = "foo::bar\n::boo as boo";
        let result = "foo::bar::boo";
        assert_eq!(result, trim_spaces_and_as(text));
    }

    #[test]
    fn it_develops_fully_1() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_all_dependencies(text);
        assert_eq!(
            result,
            Set::from([
                String::from("foo::bar1"),
                String::from("foo:: bar2"),
                String::from("foo:: bar3::far"),
                String::from("foo:: bar3:: boo")
            ])
        );
    }

    #[test]
    fn it_develops_fully_2() {
        let text = "crate::{foo::{bar}, baz, abc::def}";
        let result = develop_all_dependencies(text);
        assert_eq!(
            result,
            Set::from([
                String::from("crate:: abc::def"),
                String::from("crate:: baz"),
                String::from("crate::foo::bar")
            ])
        );
    }

    #[test]
    fn it_parses_multiple_use() {
        let text = "use foo::bar;\npub use bar::foo;\n\tuse foobar;";
        let result = parse_use(text);
        assert_eq!(
            result,
            vec![
                String::from("foo::bar"),
                String::from("bar::foo"),
                String::from("foobar")
            ]
        );
    }

    #[test]
    fn it_keeps_before_cfg_test() {
        let text = r#"
foo
#[cfg(test)]
bar
#[cfg(test)]
baz
        "#;
        let result = keep_before_cfg_test(text);
        assert_eq!(result, Some(String::from("\nfoo\n")));
    }

    #[test]
    fn it_parses_dependencies_outside_tests() {
        let text = r#"
use crate::dependencies_parser::bar as bar;
    use cargo_graphmod::dependencies_parser::{bar1,
                      bar2,
                      bar3::{abc, xyz}};
pub use crate::dependencies_parser::foobar;
use self::foobaz;
use external::aaa;

fn main() {
    crate::other::dep::fun(); // not handled
}

#[cfg(test)]
mod tests {
    use inside_tests::other; // discarded
}
        "#;
        let mut result = RustParser::parse_dependencies(
            text,
            //"cargo_graphmod",
            //vec![String::from("path"), String::from("mod")],
        );
        result.sort();
        assert_eq!(
            result,
            vec![
                DependencyPath(vec![
                    String::from("cargo_graphmod"),
                    String::from("dependencies_parser"),
                    String::from("bar1")
                ]),
                DependencyPath(vec![
                    String::from("cargo_graphmod"),
                    String::from("dependencies_parser"),
                    String::from("bar2")
                ],),
                DependencyPath(vec![
                    String::from("cargo_graphmod"),
                    String::from("dependencies_parser"),
                    String::from("bar3"),
                    String::from("abc")
                ],),
                DependencyPath(vec![
                    String::from("cargo_graphmod"),
                    String::from("dependencies_parser"),
                    String::from("bar3"),
                    String::from("xyz")
                ],),
                DependencyPath(vec![
                    String::from("crate"),
                    String::from("dependencies_parser"),
                    String::from("bar")
                ],),
                DependencyPath(vec![
                    String::from("crate"),
                    String::from("dependencies_parser"),
                    String::from("foobar")
                ]),
                DependencyPath(vec![String::from("external"), String::from("aaa")]),
                DependencyPath(vec![String::from("self"), String::from("foobaz")]),
            ]
        );
    }
}
