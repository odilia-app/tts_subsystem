use tts_subsystem::{Error, Speaker};

fn main() -> Result<(), Error> {
    let speaker = Speaker::new("spd-rust-example")?;
    for (i, module) in speaker.synthesis_voices()?.take(10).enumerate() {
        println!("{}. {:#?}", i + 1, module);
    }
    Ok(())
}
