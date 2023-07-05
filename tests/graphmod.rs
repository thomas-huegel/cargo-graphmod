use std::fs::read_to_string;

use cargo_graphmod::app_builder::run_app;

#[test]
fn it_generates_the_example_graph() {
    let output = run_app("tests/example/src", "example");
    let golden_master = read_to_string("tests/example/modules.dot").unwrap();
    assert_eq!(output.trim(), golden_master.trim());
}