use apexgrep::config_args;

#[test]
fn test_config_args() {
    let args: Vec<String> = vec![
        "testing string at position 0 ".to_string(),
        "apex".to_string(),
        "example.txt".to_string(),
    ];
    let config = config_args(&args);
    // assert_eq!(text, "apex");
    // assert_eq!(filepath, "example.txt");

    let config_result = match config {
        Ok(config) => config,
        Err(e) => panic!(" Error : {}", e),
    };
    println!("{:?}", config_result);
    assert_eq!(config_result.query, "apex");
    assert_eq!(config_result.file_path, "example.txt");
}
