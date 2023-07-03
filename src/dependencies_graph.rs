use crate::{components:: {ModuleComponents, CodeBase}, dependencies_parser::parse_dependencies, trie::Trie};

pub type DependenciesGraph<'a> = Trie<'a, String, Vec<ModuleComponents>>;

pub fn generate_trie_from_code<'a>(code: &'a CodeBase, crate_name: &str) -> DependenciesGraph<'a> {
    let mut trie = DependenciesGraph::new();
    for (name, contents) in code.0.iter() {
        trie.insert(&name.0, parse_dependencies(contents, crate_name));
    }
    trie
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use crate::{components::{CodeBase, ModuleComponents}, dependencies_graph::{generate_trie_from_code, DependenciesGraph}};

    fn make_code_base(dependencies: &[(&str, &str)]) -> CodeBase {
        dependencies.iter().map(|(path, deps)| (path.split("::").map(|s| s.to_string()).collect::<Vec<_>>().into(), deps.to_string()))
            .collect::<HashMap<_,_>>().into()
    }
    
    #[test]
    fn it_builds_the_trie() {
        let crate_name = "my_crate";
        let foobar = ("foo::bar", "use external::dep;\nuse crate::abc;\n use my_crate::def;");
        let abc = ("abc", "use crate::foo::Panel;");
        let def = ("def", "use crate::foo::bar::Widget;");
        let foomod = ("foo::mod", "pub use bar;");
        let code = make_code_base(&[foobar, abc, def, foomod]);
        let result = generate_trie_from_code(&code, crate_name);
        let sfoo = String::from("foo");
        let sbar = String::from("bar");
        let smod = String::from("mod");
        let sabc = String::from("abc");
        let sdef = String::from("def");
        let expected = DependenciesGraph { 
            value: None, 
            children: HashMap::from([
                (&sfoo, DependenciesGraph { 
                    value: None,
                    children: HashMap::from([
                        (&sbar, DependenciesGraph {
                            value: Some(vec![ModuleComponents(vec![String::from("abc")]), ModuleComponents(vec![String::from("def")])]),
                            children: HashMap::new()
                        }),
                        (&smod, DependenciesGraph { 
                            value: Some(vec![]),
                            children: HashMap::new()
                        })
                    ])
                }),
                (&sabc, DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("Panel")])]),
                    children: HashMap::new()
                }),
                (&sdef, DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("bar"), String::from("Widget")])]),
                    children: HashMap::new()
                })
            ])
        };
        assert_eq!(result, expected);
    }
}