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
#[cfg_attr(feature = "serde_derives", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde_derives", derive(serde::Serialize, serde::Deserialize))]
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
    pub fn id(&self) -> &str {
        match self {
            Self::GptJ6B => GptJ6B::ID,
            Self::Boris6B => Boris6B::ID,
            Self::FairseqGpt13B => FairseqGpt13B::ID,
            Self::Custom(custom_engine) => &custom_engine.id,
        }
    }

    /// Get the maximum amount of tokens this engine definition can have.
    pub fn max_tokens(&self) -> usize {
        self.to_custom_engine_definition().max_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_engine_definition_static() {
        let _ = CustomEngineDefinition::r#static("static", 42);
    }

    #[test]
    fn test_custom_engine_definition_dynamic() {
        let _ = CustomEngineDefinition::dynamic("dynamic".into(), 42);
    }

    #[test]
    fn test_custom_engine_definition_new() {
        let _ = CustomEngineDefinition::new("new", 42);
        let _ = CustomEngineDefinition::new(String::from("new"), 42);
    }

    #[test]
    fn test_engine_definition_to_custom_engine_definition() {
        assert_eq!(EngineDefinition::GptJ6B.to_custom_engine_definition(), Cow::Owned(GptJ6B::AS_CUSTOM_ENGINE_DEFINITION));
        assert_eq!(EngineDefinition::Boris6B.to_custom_engine_definition(), Cow::Owned(Boris6B::AS_CUSTOM_ENGINE_DEFINITION));
        assert_eq!(EngineDefinition::FairseqGpt13B.to_custom_engine_definition(), Cow::Owned(FairseqGpt13B::AS_CUSTOM_ENGINE_DEFINITION));

        let custom_engine_definition = CustomEngineDefinition::new("custom", 42);
        let custom_engine_definition_clone = custom_engine_definition.clone();
        let cow_custom_engine_definition = Cow::Borrowed(&custom_engine_definition_clone);
        assert_eq!(
            EngineDefinition::Custom(custom_engine_definition).to_custom_engine_definition(),
            cow_custom_engine_definition,
        );
    }

    #[test]
    fn test_engine_definition_id() {
        assert_eq!(EngineDefinition::GptJ6B.id(), GptJ6B::ID);
        assert_eq!(EngineDefinition::Boris6B.id(), Boris6B::ID);
        assert_eq!(EngineDefinition::FairseqGpt13B.id(), FairseqGpt13B::ID);
        assert_eq!(EngineDefinition::Custom(CustomEngineDefinition::r#static("static", 42)).id(), "static");
    }

    #[test]
    fn test_engine_definition_max_tokens() {
        assert_eq!(EngineDefinition::GptJ6B.max_tokens(), GptJ6B::MAX_TOKENS);
        assert_eq!(EngineDefinition::Boris6B.max_tokens(), Boris6B::MAX_TOKENS);
        assert_eq!(EngineDefinition::FairseqGpt13B.max_tokens(), FairseqGpt13B::MAX_TOKENS);
        assert_eq!(EngineDefinition::Custom(CustomEngineDefinition::r#static("static", 42)).max_tokens(), 42);
    }
}