Rust Wrapper for SVOX Pico TTS
===

## Introduction

This library is a very simple wrapper for the SVOX Pico TTS Linux package.

What is Pico?

The Pico Text-to-Speech (TTS) service uses the TTS binary from SVOX for producing spoken text.

Support languages:
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

```rust
use svox_pico_tts::tts::{Languages, pico2wave};

fn main() {
    pico2wave("My name is john".to_string(), "test_en.wav".to_string(), Languages::en_US);

    pico2wave("My name is john".to_string(), "test_en.wav".to_string(), Languages::en_GB);

    dbg!(pico2wave("Je m'appelle Jean".to_string(), "test_fr.wav".to_string(), Languages::fr_FR));

    dbg!(pico2wave("Mi chiamo Giuseppe".to_string(), "test_it.wav".to_string(), Languages::it_IT));

    dbg!(pico2wave("Mi nombre es alejandro".to_string(), "test_es.wav".to_string(), Languages::es_ES));
    
    dbg!(pico2wave("Ich bin Rodolf".to_string(), "test_de.wav".to_string(), Languages::de_DE));
}
```
