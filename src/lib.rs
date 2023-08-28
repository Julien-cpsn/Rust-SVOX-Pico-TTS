#[cfg(test)]
mod test;

pub mod tts {
    use std::fmt::{Display, Formatter};
    use std::process::{Command, Output};

    /// Pico TTS generator
    pub struct PicoTTS {
        pub lang: Languages,
        pub file_name: String,
        pub text: String
    }

    impl PicoTTS {
        /// Default generator
        pub fn new() -> PicoTTS {
            PicoTTS {
                lang: Languages::en_US,
                file_name: "".to_string(),
                text: "".to_string(),
            }
        }

        /// Convert generator's text to wave file
        pub fn to_wave(self) -> Output {
            let mut pico = Command::new("pico2wave");

            pico.arg(format!("--lang={}", self.lang));
            pico.arg(format!("--wave={}", self.file_name));
            pico.arg(format!("\"{}\"", self.text));

            pico.output().unwrap()
        }
    }

    /// List of available languages
    #[allow(non_camel_case_types)]
    pub enum Languages {
        en_US,
        en_GB,
        fr_FR,
        it_IT,
        es_ES,
        de_DE
    }

    impl Display for Languages {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Languages::en_US => write!(f, "en-US"),
                Languages::en_GB => write!(f, "en-GB"),
                Languages::fr_FR => write!(f, "fr-FR"),
                Languages::it_IT => write!(f, "it-IT"),
                Languages::es_ES => write!(f, "es-ES"),
                Languages::de_DE => write!(f, "de-DE")
            }
        }
    }

    /// Convert text to wave file
    pub fn pico2wave(text: String, file_name: String, lang: Languages) -> Output {
        let mut pico = Command::new("pico2wave");

        pico.arg(format!("--lang={lang}"));
        pico.arg(format!("--wave={file_name}"));
        pico.arg(format!("\"{text}\""));

        pico.output().unwrap()
    }
}