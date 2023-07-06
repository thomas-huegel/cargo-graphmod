use std::collections::{BTreeSet as Set, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

use crate::dependency_components::DependencyComponents;

const CRATE: &str = "crate";
const INPUT_SEPARATOR: &str = "::";
const MOD: &str = "mod";
const SELF: &str = "self";
const SUPER: &str = "super";

fn develop_innermost_dependencies (text: &str) -> Set<String> {
    lazy_static! {
        static ref PRODUCT: Regex = Regex::new(r"(?sm)(.*)\{(.*?)\}(.*)").unwrap();
    }
    let trimmed_text = text.chars().filter(|&c| !c.is_whitespace()).collect::<String>();
    let rewriting = PRODUCT.captures_iter(&trimmed_text)
        .flat_map(|cap| {
            cap[2].split(",").map(|x| (cap[1].to_string() + x + &cap[3].to_string()))
            .collect::<Vec<String>>()
        })
        .collect::<Set<String>>();
    if rewriting.is_empty() {
        Set::from([String::from(text)])
    } else {
        rewriting
    }
}

fn develop_all_dependencies (dependency: &str) -> Set<String> {
    let mut current_str = dependency.to_string();
    let mut current_deps = Set::from([dependency.to_string()]);
    while current_str.contains('{') {
        current_deps = current_deps.iter().flat_map(|s| develop_innermost_dependencies(s.as_str())).collect();
        current_str = current_deps.iter().fold(String::new(), |acc, elem| acc + elem);
    }
    current_deps
}

fn parse_use(text: &str) -> Vec<String> {
    lazy_static! {
        static ref USE: Regex = Regex::new(r"(?sm)^(?:\s)*(?:pub )?use (.*?);").unwrap();
    }
    USE.captures_iter(text).map(|cap| cap[1].to_string()).collect()
}

fn expand_dependency_components (dependency_components: &[String], crate_name: &str, mut source_components: Vec<String>) -> DependencyComponents {
    let fst = dependency_components.get(0).expect("A dependency should not be empty!");
    //println!("{:?} -> {:?}", source_components, dependency_components);
    if fst == crate_name || fst == CRATE { // absolute dependency
        return DependencyComponents::new(dependency_components.iter().skip(1).map(|s| s.into()).collect::<Vec<_>>(), None);
    } else { // relative dependency
        if let Some(last) = source_components.last() {
            if last == MOD {
                source_components.pop();
            }
        }
        if fst == SUPER {
            let mut deps: VecDeque<_> = dependency_components.to_owned().into();
            while let Some(fst) = deps.front() {
                if fst == SUPER {
                    deps.pop_front();
                    source_components.pop();
                } else {
                    break;
                }
            }
            source_components.append(&mut deps.iter().map(|s| s.into()).collect::<Vec<_>>());
            return DependencyComponents::new(source_components, None);            
        } else if fst == SELF {
            source_components.append(&mut dependency_components.iter().skip(1).map(|s| s.into()).collect::<Vec<_>>());
            return DependencyComponents::new(source_components, None);
        } else {
            //println!("{:?} -> {:?} => {}", source_components, dependency_components, is_certainly_internal);
            return DependencyComponents::new(dependency_components.iter().map(|s| s.into()).collect::<Vec<_>>(), Some(source_components));
        }
    }
}

pub fn parse_dependencies (contents: &str, crate_name: &str, source_components: Vec<String>) -> Vec<DependencyComponents> {
    parse_use(contents).iter()
        .flat_map(|s| develop_all_dependencies(&s))
        .map(|s| s.split(INPUT_SEPARATOR).map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|c| expand_dependency_components(&c, crate_name, source_components.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet as Set;

    use crate::{dependencies_parser::{develop_innermost_dependencies, develop_all_dependencies, parse_use, expand_dependency_components, parse_dependencies},
        dependency_components::DependencyComponents};
    
    #[test]
    fn it_develops_innermost() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, Set::from([String::from("foo::{bar1,bar2,bar3::far}"), String::from("foo::{bar1,bar2,bar3::boo}")]));
    }


    #[test]
    fn it_develops_innermost_2() {
        let text = "crate::{foo::{bar}, baz, abc::def}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, Set::from([String::from("crate::{foo::bar,baz,abc::def}")]));
    }

    #[test]
    fn it_swallows_newlines() {
        let text = "foo::{bar1, bar2, bar3::\n{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, Set::from([String::from("foo::{bar1,bar2,bar3::far}"), String::from("foo::{bar1,bar2,bar3::boo}")]));
    }

    #[test]
    fn it_does_nothing() {
        let text = "foo::bar";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, Set::from([String::from("foo::bar")]));
    }

    #[test]
    fn it_develops_fully_1() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_all_dependencies(text);
        assert_eq!(result, Set::from([String::from("foo::bar1"), String::from("foo::bar2"), String::from("foo::bar3::far"), String::from("foo::bar3::boo")]));
    }

    #[test]
    fn it_develops_fully_2() {
        let text = "crate::{foo::{bar}, baz, abc::def}";
        let result = develop_all_dependencies(text);
        assert_eq!(result, Set::from([String::from("crate::abc::def"), String::from("crate::baz"), String::from("crate::foo::bar")]));
    }

    #[test]
    fn it_parses_multiple_use() {
        let text = "use foo::bar;\npub use bar::foo;\n\tuse foobar;";
        let result = parse_use(text);
        assert_eq!(result, vec![String::from("foo::bar"), String::from("bar::foo"), String::from("foobar")]);
    }

    #[test]
    fn it_belongs_to_my_crate() {
        let dependency = vec![String::from("my_crate"), String::from("foo"), String::from("bar")];
        let result = expand_dependency_components(&dependency, "my_crate", vec![]);
        assert_eq!(result, DependencyComponents::new(vec![String::from("foo"), String::from("bar")], None));
    }

    #[test]
    fn it_belongs_to_crate() {
        let dependency = vec![String::from("crate"), String::from("foo"), String::from("bar")];
        let result = expand_dependency_components(&dependency, "my_crate", vec![]);
        assert_eq!(result, DependencyComponents::new(vec![String::from("foo"), String::from("bar")], None));
    }

    #[test]
    fn it_belongs_to_a_supermodule() {
        let dependency = vec![String::from("super"), String::from("super"), String::from("foo"), String::from("bar")];
        let result = expand_dependency_components(&dependency, "my_crate", vec![String::from("aaa"), String::from("bbb"), String::from("ccc"), String::from("mod")]);
        assert_eq!(result, DependencyComponents::new(vec![String::from("aaa"), String::from("foo"), String::from("bar")], None));
    }

    #[test]
    fn it_belongs_to_a_submodule_via_self() {
        let dependency = vec![String::from("self"), String::from("foo"), String::from("bar")];
        let result = expand_dependency_components(&dependency, "my_crate", vec![String::from("path"), String::from("mod")]);
        assert_eq!(result, DependencyComponents::new(vec![String::from("path"), String::from("foo"), String::from("bar")], None));
    }

    #[test]
    fn it_may_belong_to_some_external_crate() {
        let dependency = vec![String::from("foo"), String::from("bar")];
        let result = expand_dependency_components(&dependency, "my_crate", vec![String::from("path"), String::from("mod")]);
        assert_eq!(result, DependencyComponents::new(vec![String::from("foo"), String::from("bar")], Some(vec![String::from("path")])));
    }

    #[test]
    fn it_parses_dependencies() {
        let text = r#"
use crate::foo::bar;
    use my_crate::foo::{bar1,
                      bar2,
                      bar3::{abc, xyz}};
pub use crate::foo1;
use self::foobaz;
use external::aaa;

fn main() {
}
        "#;
        let mut result = parse_dependencies(text, "my_crate", vec![String::from("path"), String::from("mod")]);
        result.sort();
        assert_eq!(result, vec![
            DependencyComponents::new(vec![String::from("external"), String::from("aaa")], Some(vec![String::from("path")])),
            DependencyComponents::new(vec![String::from("foo"), String::from("bar")], None),
            DependencyComponents::new(vec![String::from("foo"), String::from("bar1")], None),
            DependencyComponents::new(vec![String::from("foo"), String::from("bar2")], None),
            DependencyComponents::new(vec![String::from("foo"), String::from("bar3"), String::from("abc")], None),
            DependencyComponents::new(vec![String::from("foo"), String::from("bar3"), String::from("xyz")], None),
            DependencyComponents::new(vec![String::from("foo1")], None),
            DependencyComponents::new(vec![String::from("path"), String::from("foobaz")], None),
        ]);
    }
}

