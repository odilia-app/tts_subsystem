use tts_subsystem::{Error, Priority, Speaker};

fn main() -> Result<(), Error> {
    let speaker = Speaker::new("spd-rust-example")?;
    speaker.speak_char(Priority::Message, '💯')?;
    speaker.speak_char(Priority::Message, 'a')?;
    speaker.speak_char(Priority::Message, 'A')?;
    Ok(())
}
