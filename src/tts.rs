use speech_dispatcher_sys as spd;
use crate::errors;
pub mod tts {
    pub struct Speaker {
    Connection:speech_dispatcher_sys::SPDConnection,
    }
impl Speaker {
    pub fn init(app_name:&str)->Result<Speaker, errors::error>{
        let con=speech_dispatcher_sys::SPDOpen(app_name.as_ptr(), "main", None, speech_dispatcher_sys::SPD_MODE_SINGLE);
        if con.is_null{
            err(errors::error::SpeechError)
        }
        speaker=Speaker{con as *SPDConnection}
        Ok(speaker)    
    }
}
}