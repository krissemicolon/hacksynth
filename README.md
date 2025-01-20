# hacksynth
> Synthesizer Project of Lian Studer & Kris Huber for `[yth22]`

![Neubad](assets/neubad.png)

> [!NOTE]  
> As of now this synth only takes input from MIDI Devices. Meaning you need to connect a MIDI Keyboard to play notes.

## Features  
- **Graphical User Interface**
- **Velocity-Sensitive MIDI Keyboard Input** – Responds to touch dynamics for expressive playing.  
- **Dual Oscillators with Selectable Waveforms:**  
  - Sine  
  - Triangle  
  - Sawtooth  
  - Square  
- **Two Low-Pass Filters** – Shape your sound.  
- **Independent ADSR Envelopes per Oscillator** – Fine-tune attack, decay, sustain, and release settings individually.  
- **Detune Control** – Adjust pitch variations for a richer, wider sound.  

## Sound Demo
![Demo](assets/demo.wav)

## Installation
You can either download the latest binary in the [releases](https://github.com/krissemicolon/hacksynth/releases) or as described here build from source.

1. Clone the repository
```sh
git clone https://github.com/krissemicolon/hacksynth
cd hacksynth
```

2. Compile
```sh
cargo build --release
```

2.1. (Optional, macOS) Build a `Hacksynth.app`
> Requires cargo-bundle install it with `cargo install cargo-bundle`
```sh
cargo bundle --release
```

3. Running
```sh
./target/release/hacksynth
```
or if you built a `.app`
```sh
open target/release/bundle/osx/Hacksynth.app
```
