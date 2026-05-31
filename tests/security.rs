use camino::Utf8Path;

#[test]
fn rejects_path_traversal() {
    let result = newgpa::security::paths::reject_path_traversal(Utf8Path::new("../secret"));
    assert!(result.is_err());
}
