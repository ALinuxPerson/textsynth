//! Types and operations involving engine definitions.

mod private {
    pub trait Sealed {}
}

use std::borrow::Cow;

/// Declares that the implementing type represents a already known engine definition. This trait is
/// sealed and cannot be implemented for types outside of this crate.
pub trait KnownEngineDefinition: private::Sealed {
    /// The id of this engine definition.
    const ID: &'static str;

    /// The maximum amount of tokens this engine definition can have.
    const MAX_TOKENS: usize = 1024;

    /// Conversion into a [`CustomEngineDefinition`].
    const AS_CUSTOM_ENGINE_DEFINITION: CustomEngineDefinition =
        CustomEngineDefinition::r#static(Self::ID, Self::MAX_TOKENS);
}

/// [GPT-J] is a language model with 6 billion parameters trained on [the Pile] (825 GB of text data)
/// published by [EleutherAI]. Its main language is English but it is also fluent in several other
/// languages. It is also trained on several computer languages.
///
/// [GPT-J]: https://github.com/kingoflolz/mesh-transformer-jax/#gpt-j-6b
/// [the Pile]: https://pile.eleuther.ai
/// [EleutherAI]: https://eleuther.ai
pub struct GptJ6B {
    _priv: (),
}

impl KnownEngineDefinition for GptJ6B {
    const ID: &'static str = "gptj_6B";
    const MAX_TOKENS: usize = 2048;
}

impl private::Sealed for GptJ6B {}

/// [Boris] is a fine tuned version of GPT-J for the French language. Use this model is you want the
/// best performance with the French language.
///
/// [Boris]: https://github.com/coteries/cedille-ai
pub struct Boris6B {
    _priv: (),
}

impl KnownEngineDefinition for Boris6B {
    const ID: &'static str = "boris_6B";
}

impl private::Sealed for Boris6B {}

/// [Fairseq GPT 13B] is the largest publicly available English model with 13 billion parameters.
/// Its training corpus is less diverse than GPT-J but it has better performance at least on pure
/// English language tasks.
///
/// # Notes
/// Support of this model is still experimental. It may stop working without notice.
///
/// [Fairseq GPT 13B]: https://github.com/pytorch/fairseq/tree/main/examples/moe_lm
pub struct FairseqGpt13B {
    _priv: (),
}

impl KnownEngineDefinition for FairseqGpt13B {
    const ID: &'static str = "fairseq_gpt_13B";
}

impl private::Sealed for FairseqGpt13B {}

/// A custom engine definition which may or may not exist.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CustomEngineDefinition {
    /// The id of this engine definition.
    pub id: Cow<'static, str>,

    /// The maximum amount of tokens this engine definition can have.
    pub max_tokens: usize,
}

impl CustomEngineDefinition {
    /// Creates a new custom engine definition with the given statically known id and maximum amount
    /// of tokens.
    pub const fn r#static(id: &'static str, max_tokens: usize) -> Self {
        Self {
            id: Cow::Borrowed(id),
            max_tokens,
        }
    }

    /// Creates a new custom engine definition with the given runtime known id and maximum amount of
    /// tokens.
    pub const fn dynamic(id: String, max_tokens: usize) -> Self {
        Self {
            id: Cow::Owned(id),
            max_tokens,
        }
    }

    /// Creates a new custom engine definition with the given id and maximum amount of tokens.
    pub fn new(id: impl Into<Cow<'static, str>>, max_tokens: usize) -> Self {
        Self {
            id: id.into(),
            max_tokens,
        }
    }
}

/// Engine definitions supported by this crate.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EngineDefinition {
    /// See [`GptJ6B`] for documentation.
    GptJ6B,

    /// See [`Boris6B`] for documentation.
    Boris6B,

    /// See [`FairseqGpt13B`] for documentation.
    FairseqGpt13B,

    /// A custom engine definition.
    Custom(CustomEngineDefinition),
}

impl EngineDefinition {
    /// Convert this engine definition into a [`CustomEngineDefinition`].
    pub const fn to_custom_engine_definition(&self) -> Cow<CustomEngineDefinition> {
        match self {
            Self::GptJ6B => Cow::Owned(GptJ6B::AS_CUSTOM_ENGINE_DEFINITION),
            Self::Boris6B => Cow::Owned(Boris6B::AS_CUSTOM_ENGINE_DEFINITION),
            Self::FairseqGpt13B => Cow::Owned(FairseqGpt13B::AS_CUSTOM_ENGINE_DEFINITION),
            Self::Custom(custom_engine) => Cow::Borrowed(custom_engine),
        }
    }

    /// Get the id of this engine definition.
    pub fn id(&self) -> Cow<str> {
        match self {
            Self::GptJ6B => Cow::Borrowed(GptJ6B::ID),
            Self::Boris6B => Cow::Borrowed(Boris6B::ID),
            Self::FairseqGpt13B => Cow::Borrowed(FairseqGpt13B::ID),
            Self::Custom(custom_engine) => Cow::Borrowed(&custom_engine.id),
        }
    }

    /// Get the maximum amount of tokens this engine definition can have.
    pub fn max_tokens(&self) -> usize {
        self.to_custom_engine_definition().max_tokens
    }
}
