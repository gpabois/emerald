#[macro_export]
macro_rules! test_case {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/", $fname) // assumes Linux ('/')!
    };
}

#[macro_export]
macro_rules! test_emerald {
    () => {
        test_case!("jewel")
    };
}
