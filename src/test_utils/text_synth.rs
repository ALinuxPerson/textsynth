use crate::core::TextSynth;
use crate::engine::Engine;
use crate::prelude::EngineDefinition;
use once_cell::sync::{Lazy, OnceCell};

pub const ENGINE_DEFINITION: EngineDefinition = EngineDefinition::GptJ6B;
static TEXT_SYNTH: OnceCell<TextSynth> = OnceCell::new();
static ENGINE: Lazy<Engine> = Lazy::new(|| get().engine(ENGINE_DEFINITION));

pub fn get() -> &'static TextSynth {
    TEXT_SYNTH.get_or_init(|| TextSynth::new(super::api_key().into()))
}

pub fn engine() -> &'static Engine<'static> {
    &ENGINE
}
