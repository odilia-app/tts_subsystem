use crate::{iter, Error};
use speech_dispatcher_sys as spd;
use std::{
    ffi::CString,
    fmt,
    ptr::{self, NonNull},
};

//macros for defining functions in shorter code and with less repetition
macro_rules! spd_return_err_if_fail {
    ($res: ident, $err: ident) => {
        if $res == -1 {
            Err(Error::$err)
        } else {
            Ok(())
        }
    };
}

//enums for tts and speech dispatcher specific things
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectionMode {
    Single = spd::SPDConnectionMode::SPD_MODE_SINGLE,
    Threaded = spd::SPDConnectionMode::SPD_MODE_THREADED,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Priority {
    Important = spd::SPDPriority::SPD_IMPORTANT,
    Message = spd::SPDPriority::SPD_MESSAGE,
    Notification = spd::SPDPriority::SPD_NOTIFICATION,
    Progress = spd::SPDPriority::SPD_PROGRESS,
    Text = spd::SPDPriority::SPD_TEXT,
}

#[derive(Debug, PartialEq)]
pub struct Speaker {
    con: NonNull<spd::SPDConnection>,
}

impl Speaker {
    pub fn new(app_name: &str) -> Result<Speaker, Error> {
        let name = CString::new(app_name).expect("name shouldn't contain null bytes");
        let con = unsafe {
            spd::spd_open(
                name.as_ptr().cast(),
                ptr::null(),
                ptr::null(),
                ConnectionMode::Single as u32,
            )
        };
        NonNull::new(con)
            .map(|con| Self { con })
            .ok_or(Error::InitError)
    }

    pub fn speak(&self, priority: Priority, text: &str) -> Result<(), Error> {
        let text = CString::new(text).expect("text shouldn't contain null bytes");
        let priority = priority as u32;
        let res = unsafe { spd::spd_say(self.con.as_ptr(), priority, text.as_ptr().cast()) };
        spd_return_err_if_fail!(res, SpeechSynthError)
    }

    pub fn speak_char(&self, priority: Priority, c: char) -> Result<(), Error> {
        // I'd love to ues spd_wchar to make this easier, but it seems that it doesn't work, at
        // least not with espeak-ng. (mcb2003 <mikey@blindcomputing.org>)
        // This is 4-bytes for the char plus a null terminator
        let mut buf = [0u8; 5];
        // Encode the character into `buf` as UTF8.
        // We don't need to manually terminate this because `buf` was pre-filled with zeros.
        c.encode_utf8(&mut buf);

        let priority = priority as u32;
        let res = unsafe { spd::spd_char(self.con.as_ptr(), priority, buf.as_ptr().cast()) };
        spd_return_err_if_fail!(res, SpeechSynthError)
    }

    pub fn speak_key(&self, priority: Priority, key_name: &str) -> Result<(), Error> {
        let key_name = CString::new(key_name).expect("key_name shouldn't contain null bytes");
        let priority = priority as u32;
        let res = unsafe { spd::spd_key(self.con.as_ptr(), priority, key_name.as_ptr().cast()) };
        spd_return_err_if_fail!(res, SpeechSynthError)
    }

    pub fn stop(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_stop(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, StopSpeechError)
    }

    pub fn stop_all(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_stop_all(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, StopSpeechError)
    }

    pub fn pause(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_pause(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn pause_all(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_pause_all(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn resume(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_resume(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn resume_all(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_resume_all(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn cancel(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_cancel(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, SpeechCancelationError)
    }

    pub fn cancel_all(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_cancel_all(self.con.as_ptr()) };
        spd_return_err_if_fail!(res, SpeechCancelationError)
    }

    pub fn output_modules(&self) -> Result<iter::OutputModuleIter, Error> {
        let res = NonNull::new(unsafe { spd::spd_list_modules(self.con.as_ptr()) })
            .ok_or(Error::ListModulesError)?;
        Ok(iter::OutputModuleIter::new(res))
    }

    pub fn synthesis_voices(&self) -> Result<iter::SynthesisVoiceIter, Error> {
        let res = NonNull::new(unsafe { spd::spd_list_synthesis_voices(self.con.as_ptr()) })
            .ok_or(Error::ListSynthesisVoicesError)?;
        Ok(iter::SynthesisVoiceIter::new(res))
    }
    pub fn rate(&self) -> i32 {
        let res = unsafe { spd::spd_get_voice_rate(self.con.as_ptr()) };
        res
    }

    pub fn set_rate(&mut self, value: i32) -> Result<(), Error> {
        let res = unsafe { spd::spd_set_voice_rate(self.con.as_mut(), value) };
        spd_return_err_if_fail!(res, SynthParamError)
    }
    pub fn volume(&self) -> i32 {
        let res = unsafe { spd::spd_get_volume(self.con.as_ptr()) };
        res
    }

    pub fn set_volume(&mut self, value: i32) -> Result<(), Error> {
        let res = unsafe { spd::spd_set_volume(self.con.as_mut(), value) };
        spd_return_err_if_fail!(res, SynthParamError)
    }
    pub fn pitch(&self) -> i32 {
        let res = unsafe { spd::spd_get_voice_pitch(self.con.as_ptr()) };
        res
    }

    pub fn set_pitch(&mut self, value: i32) -> Result<(), Error> {
        let res = unsafe { spd::spd_set_voice_pitch(self.con.as_mut(), value) };
        spd_return_err_if_fail!(res, SynthParamError)
    }
    pub fn set_pitch_range(&mut self, value: i32) -> Result<(), Error> {
        let res = unsafe { spd::spd_set_voice_pitch_range(self.con.as_mut(), value) };
        spd_return_err_if_fail!(res, SynthParamError)
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        unsafe {
            spd::spd_close(self.con.as_mut());
        }
    }
}
unsafe impl Send for Speaker {
    
}
impl fmt::Write for Speaker {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.speak(Priority::Text, text).map_err(|_| fmt::Error)
    }
}
