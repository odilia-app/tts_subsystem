use std::{borrow::Cow, ffi::CStr};

use speech_dispatcher_sys as spd;

#[derive(Clone, Debug)]
pub struct Voice<'a> {
    name: Cow<'a, str>,
    language: Cow<'a, str>,
    variant: Cow<'a, str>,
}

impl Voice<'_> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn variant(&self) -> &str {
        &self.variant
    }
}

impl From<spd::SPDVoice> for Voice<'_> {
    fn from(voice: spd::SPDVoice) -> Self {
        let name = unsafe { CStr::from_ptr(voice.name) }.to_string_lossy();
        let language = unsafe { CStr::from_ptr(voice.language) }.to_string_lossy();
        let variant = unsafe { CStr::from_ptr(voice.variant) }.to_string_lossy();
        Self {
            name,
            language,
            variant,
        }
    }
}
