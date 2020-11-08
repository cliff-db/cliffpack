use super::*;
use std::path::Path;

#[test]
fn test_create() {
    let test_file = Path::new(".gitignore").to_path_buf();
    let output_file = Path::new("./target/test.cliffpl");

    let mut creator = super::PluginCreator::new(PluginMeta {
        entrypoint: "lol".to_string(),
        name: "Test Plugin".to_string(),
        version: "1.0.0".to_string()
    });

    creator.add_file(test_file);
    creator.create_plugin(output_file).unwrap();

    assert_eq!(1 + 2, 3);
}
