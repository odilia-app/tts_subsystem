
    #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
    #[non_exhaustive]
    pub enum Error {
        #[error("unable to initialise speech dispatcher")]
        InitError,
        #[error("error synthesizing text")]
        SpeechSynthError,
        #[error("unable to stop currently spoken message. Perhaps there's no message being spoken?")]
        StopSpeechError,
        #[error("unable to cancel speech")]
        SpeechCancelationError,
        #[error("error pausing or resuming speech")]
        TTSPauseResumeError,
        #[error("problems setting a speech param")]
        SynthParamError,
        #[error("a weird unknown error")]
        Unknown,
    }
