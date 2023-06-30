use std::{path::Path, 
    fs::read_to_string, io::Result, collections::HashSet};

use crate::{components:: {ModuleComponents, DependenciesGraph, SEPARATOR, CRATE}, modules_trie};

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

fn parse_dependencies (text: &str, crate_name: &str) -> Vec<ModuleComponents> {
    parse_use(text).iter()
        .flat_map(|s| develop_all_dependencies(&s))
        .map(|s| s.split(SEPARATOR).map(|s| s.to_string()).collect::<Vec<String>>())
        .filter_map(|c| belongs_to_crate(&c, crate_name))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{dependencies_graph::{develop_innermost_dependencies, develop_all_dependencies, parse_use, belongs_to_crate, parse_dependencies}, components::ModuleComponents};
    
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

pub fn generate_graph(path: &Path, graph: &mut DependenciesGraph, crate_name: &str, skip_length: usize) -> Result<()> {
    if path.is_file() {
        if let Some(Some("rs")) = path.extension().map(|e| e.to_str()) {
            let contents = read_to_string(path)?;
            let components: ModuleComponents = path.with_extension("").iter()
                .skip(skip_length)
                .map(|s| s.to_string_lossy().into())
                .filter(|s| s != "mod")
                .collect::<Vec<String>>().into();
            graph.map.insert(
                components.clone(), 
                parse_dependencies(&contents, crate_name)
            );
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                generate_graph(&entry.path(), graph, crate_name, skip_length)?;
            }
        }
    }
    Ok(())
}

fn lookup (dependency: &ModuleComponents, graph: &DependenciesGraph) -> Option<ModuleComponents> {
    let mut components = dependency.clone();
    while graph.map.get(&components).is_none() && ! components.0.is_empty() {
        components.0.pop();
    }
    if components.0.is_empty() {
        None
    } else {
        Some (components)
    }
}

pub fn format_graph (graph: DependenciesGraph) -> String {
    //let index: ModulesIndex = graph.set.into_iter().zip((0..).map(Index)).collect::<HashMap<_,_>>().into();
    let trie = modules_trie::convert(&graph);
    /* let vertices: String = index.0.iter()
        .map(|(components, idx)| String::from("u") + &idx.0.to_string() + "[label=\"" + &components.0.join("::") + "\"]")
        .collect::<Vec<_>>()
        .join("\n"); */
    let arcs: String = graph.map.iter()
        .map(|(k, values)| values.iter()
            //.filter_map(|v| index.get(v).map(|idx| String::from("u") + &index.get(&k).unwrap().to_string() + " -> " + "u" + &idx.to_string()))
            .filter_map(|v| lookup(v, &graph).map(|trimmed| format!("{}", k) + " -> " + &format!("{}", trimmed)))
            //.map(|v| format!("{}", k) + " -> " + &format!("{}", v))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join("\n"))
        .filter(|s| s != "")
        .collect::<Vec<_>>()
        .join("\n");    
    String::from("digraph G {\n\n") + &format!("{}", trie) + &arcs + "\n}\n"
}