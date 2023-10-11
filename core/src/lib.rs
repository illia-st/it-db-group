pub mod row;
pub mod types;
pub mod scheme;
pub mod table;
pub mod db;

#[macro_export]
macro_rules! test_resources {
        ($fname:expr) => (
            concat!(env!("CARGO_MANIFEST_DIR"), "/test_resources/", $fname)
        )
}