Rust Wrapper for SVOX Pico TTS
===

[![Rust](https://github.com/Julien-cpsn/Rust-SVOX-Pico-TTS/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/Rust-SVOX-Pico-TTS/actions/workflows/rust.yml)

## Introduction

This library is a very simple wrapper for the SVOX Pico TTS Linux package.

**What is Pico TTS?**

Pico is a Text-To-Speech (TTS) voice sinthesizer. It uses the TTS binary from SVOX for producing spoken text.

**Supported languages:**
- English, US (en-US)
- English, GB (en-GB)
- French (fr-FR)
- Italian (it-IT)
- German (de-DE)
- Spanish (es-ES)

## Prerequisites

Having `libttspico-utils` installed on your machine

For *Ubuntu*:

```shell
sudo apt-get install libttspico-utils
```

For *other Linux distros* (thanks to [@naggety](https://github.com/nagget)):

https://github.com/naggety/picotts

## Usage example

There are 2 main approaches to use this library depending on your use case, either function or struct.
Both work the same.

```rust
use svox_pico_tts::tts::{Languages, pico2wave};

fn main() {
    // Function approach
    pico2wave("My name is john".to_string(), "test_en.wav".to_string(), Languages::en_US);
    pico2wave("My name is william".to_string(), "test_en.wav".to_string(), Languages::en_GB);
    pico2wave("Je m'appelle Jean".to_string(), "test_fr.wav".to_string(), Languages::fr_FR);
    pico2wave("Mi chiamo Giuseppe".to_string(), "test_it.wav".to_string(), Languages::it_IT);
    pico2wave("Mi nombre es alejandro".to_string(), "test_es.wav".to_string(), Languages::es_ES);
    pico2wave("Ich bin Rodolf".to_string(), "test_de.wav".to_string(), Languages::de_DE);
    
    // Struct approach
    let mut generator = PicoTTS::new();
    generator.text = "My name is john".to_string();
    generator.lang = Languages::en_US;
    generator.file_name = "test_en_2.wav".to_string();
    let output = generator.to_wave();
    
    // Alt struct approach
    let generator = PicoTTS {
        text: "My name is john".to_string(),
        lang: Languages::en_US,
        file_name: "test_en_3.wav".to_string()
    };

    let output = generator.to_wave();
}
```
