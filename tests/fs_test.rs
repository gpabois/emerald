macro_rules! test_case {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/", $fname) // assumes Linux ('/')!
    };
}

macro_rules! test_jewel {
    () => {
        test_case!("jewel")
    };
}

#[test]
/// Test the canonicalization of jewel paths
fn test_valid_canonicalize() {
    let root = test_jewel!();
    let jewel = emerald::open(&root).unwrap();

    let local_path = emerald::path::Path::new("/index.md").unwrap();
    let expected = std::path::Path::new(root).join("index.md");
    let canon = emerald::fs::canonicalize(&jewel, &local_path).unwrap();

    assert_eq!(canon, expected);
}

#[test]
fn test_valid_read_dir() {
    let root = test_jewel!();
    let jewel = emerald::open(&root).unwrap();

    let entries = emerald::fs::read_dir(&jewel, &emerald::path::Path::default())
        .unwrap()
        .collect::<Vec<_>>();
}
