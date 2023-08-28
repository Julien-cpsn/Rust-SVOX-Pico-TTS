use crate::tts::{Languages, pico2wave, PicoTTS};

#[test]
fn function_approach() {
    let output = pico2wave("My name is john".to_string(), "test_en_1.wav".to_string(), Languages::en_US);

    assert_eq!(0, output.status.code().unwrap());
}

#[cfg(test)]
#[test]
fn struct_approach() {
    let mut generator = PicoTTS::new();

    generator.text = "My name is john".to_string();
    generator.lang = Languages::en_US;
    generator.file_name = "test_en_2.wav".to_string();

    let output = generator.to_wave();

    assert_eq!(0, output.status.code().unwrap());
}