


mod api;
mod adapter;
mod error;


pub use adapter::Adapter;
pub use error::Error;


pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_test() {
        let adapter = api::Adapter::new("/dev/ttyACM0");
        assert!(adapter.is_ok());

        let mut adapter = adapter.unwrap();
        adapter.open();
        adapter.close();

    }
}