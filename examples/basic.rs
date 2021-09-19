use tts_subsystem::{Error, Priority, Speaker};

fn main() -> Result<(), Error> {
    let speaker = Speaker::new("spd-rust-example")?;
    speaker.speak(Priority::Message, "Test")?;
    Ok(())
}
