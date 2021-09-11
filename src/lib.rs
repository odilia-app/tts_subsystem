mod tts;
pub use tts::Speaker;
mod  errors;
pub use errors::Error;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]    
    fn succesfull_init(){
        let res=Speaker::init("tester");
        assert_eq!(res, Ok);
    }
    #[test]
    #[should_panic(expected="using this initialiser with a slice of characters with null bytes in it should fail")]
    fn this_test_should_fail(){
        let speech=Speaker::init("tes\0ter");
    }
}
