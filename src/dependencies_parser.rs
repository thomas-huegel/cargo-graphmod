use std::collections::HashSet;

use crate::components::{ModuleComponents, SEPARATOR, CRATE};

use lazy_static::lazy_static;
use regex::Regex;

fn develop_innermost_dependencies (text: &str) -> HashSet<String> {
    lazy_static! {
        static ref PRODUCT: Regex = Regex::new(r"(?sm)(.*)\{(.*?)\}(.*)").unwrap();
    }
    let trimmed_text = text.chars().filter(|&c| !c.is_whitespace()).collect::<String>();
    let rewriting = PRODUCT.captures_iter(&trimmed_text)
        .flat_map(|cap| {
            cap[2].split(",").map(|x| (cap[1].to_string() + x + &cap[3].to_string()))
            .collect::<Vec<String>>()
        })
        .collect::<HashSet<String>>();
    if rewriting.is_empty() {
        HashSet::from([String::from(text)])
    } else {
        rewriting
    }
}

fn develop_all_dependencies (dependency: &str) -> HashSet<String> {
    let mut old_length = 0;
    let mut new_length = 1;
    let mut current_deps = HashSet::from([dependency.to_string()]);
    while old_length != new_length {
        old_length = current_deps.len();
        current_deps = current_deps.iter().flat_map(|s| develop_innermost_dependencies(s.as_str())).collect();
        new_length = current_deps.len();
    }
    current_deps
}

fn parse_use(text: &str) -> Vec<String> {
    lazy_static! {
        static ref USE: Regex = Regex::new(r"(?sm)^(?:pub )?use (.*?);").unwrap();
    }
    USE.captures_iter(text).map(|cap| cap[1].to_string()).collect()
}

fn belongs_to_crate (components: &[String], crate_name: &str) -> Option<ModuleComponents> {
    if let Some(fst) = components.get(0) {
        if fst == crate_name || fst == CRATE {
            return Some(components.iter().skip(1).map(|s| s.into()).collect::<Vec<String>>().into());
        }
    }
    None 
}

pub fn parse_dependencies (text: &str, crate_name: &str) -> Vec<ModuleComponents> {
    parse_use(text).iter()
        .flat_map(|s| develop_all_dependencies(&s))
        .map(|s| s.split(SEPARATOR).map(|s| s.to_string()).collect::<Vec<String>>())
        .filter_map(|c| belongs_to_crate(&c, crate_name))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{dependencies_parser::{develop_innermost_dependencies, develop_all_dependencies, parse_use, belongs_to_crate, parse_dependencies}, components::ModuleComponents};
    
    #[test]
    fn it_develops_innermost() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, HashSet::from([String::from("foo::{bar1,bar2,bar3::far}"), String::from("foo::{bar1,bar2,bar3::boo}")]));
    }

    #[test]
    fn it_swallows_newlines() {
        let text = "foo::{bar1, bar2, bar3::\n{far, boo}}";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, HashSet::from([String::from("foo::{bar1,bar2,bar3::far}"), String::from("foo::{bar1,bar2,bar3::boo}")]));
    }

    #[test]
    fn it_does_nothing() {
        let text = "foo::bar";
        let result = develop_innermost_dependencies(text);
        assert_eq!(result, HashSet::from([String::from("foo::bar")]));
    }

    #[test]
    fn it_develops_fully() {
        let text = "foo::{bar1, bar2, bar3::{far, boo}}";
        let result = develop_all_dependencies(text);
        assert_eq!(result, HashSet::from([String::from("foo::bar1"), String::from("foo::bar2"), String::from("foo::bar3::far"), String::from("foo::bar3::boo")]));
    }

    #[test]
    fn it_parses_multiple_use() {
        let text = "use foo::bar;\npub use bar::foo;";
        let result = parse_use(text);
        assert_eq!(result, vec![String::from("foo::bar"), String::from("bar::foo")]);
    }

    #[test]
    fn it_belongs_to_my_crate() {
        let dependency = vec![String::from("my_crate"), String::from("foo"), String::from("bar")];
        let result = belongs_to_crate(&dependency, "my_crate");
        assert_eq!(result, Some(ModuleComponents(vec![String::from("foo"), String::from("bar")])));
    }

    #[test]
    fn it_belongs_to_crate() {
        let dependency = vec![String::from("crate"), String::from("foo"), String::from("bar")];
        let result = belongs_to_crate(&dependency, "my_crate");
        assert_eq!(result, Some(ModuleComponents(vec![String::from("foo"), String::from("bar")])));
    }

    #[test]
    fn it_does_not_belong_to_my_crate() {
        let dependency = vec![String::from("scratch"), String::from("foo"), String::from("bar")];
        let result = belongs_to_crate(&dependency, "my_crate");
        assert_eq!(result, None);
    }

    #[test]
    fn it_parses_dependencies() {
        let text = r#"
use crate::foo::bar;
use my_crate::foo::{bar1,
                  bar2,
                  bar3::{abc, xyz}};
pub use crate::foo1;
use external::crate::aaa;

fn main() {
}
        "#;
        let mut result = parse_dependencies(text, "my_crate");
        result.sort();
        assert_eq!(result, vec![
            ModuleComponents(vec![String::from("foo"), String::from("bar")]),
            ModuleComponents(vec![String::from("foo"), String::from("bar1")]),
            ModuleComponents(vec![String::from("foo"), String::from("bar2")]),
            ModuleComponents(vec![String::from("foo"), String::from("bar3"), String::from("abc")]),
            ModuleComponents(vec![String::from("foo"), String::from("bar3"), String::from("xyz")]),
            ModuleComponents(vec![String::from("foo1")]),
        ]);
    }
}

