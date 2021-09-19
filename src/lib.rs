pub mod tts;
pub use tts::{Priority, Speaker};
mod errors;
pub use errors::Error;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn succesfull_init() {
        let res = Speaker::new("tester");
        assert_eq!(res.is_err(), false)
    }
    #[test]
    #[should_panic(expected = "should not contain null bytes")]
    fn this_test_should_fail() {
        let _speech = Speaker::new("tes\0ter");
    }
}
