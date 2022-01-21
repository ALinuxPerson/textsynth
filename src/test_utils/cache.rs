use once_cell::sync::{Lazy, OnceCell};
use crate::prelude::{LogProbabilities, NonEmptyString};
use crate::test_utils::text_synth;

#[allow(unused_macros)]
macro_rules! fallible_cache {
    (
        $vis:vis mod $cache_name:ident {
            $(use $item:path;)*

            type Target = $target:ty;
            type Error = $error:ty;
            const INITIALIZER = $initializer:expr;
        }
    ) => {
        #[allow(dead_code)]
        $vis mod $cache_name {
            use once_cell::sync::OnceCell;
            $(use $item;)*

            static ITEM: OnceCell<$target> = OnceCell::new();
            static ERROR: OnceCell<$error> = OnceCell::new();

            fn initializer() -> Result<$target, $error> {
                $initializer
            }

            pub fn poisoned() -> bool {
                ERROR.get().is_some()
            }

            pub fn initialized() -> bool {
                ITEM.get().is_some()
            }

            pub fn get() -> &'static $target {
                if let Some(item) = ITEM.get() {
                    item
                } else if !initialized() {
                    match ITEM.get_or_try_init(initializer) {
                        Ok(item) => item,
                        Err(error) => {
                            let message = format!("initialization failure: {error}");
                            let _ = ERROR.set(error);
                            panic!("{message}")
                        }
                    }
                } else if let Some(error) = ERROR.get() {
                    panic!("initialization failure, poisoned: {error}")
                } else {
                    unreachable!()
                }
            }
        }
    };
}

static LOG_PROBABILITIES: OnceCell<LogProbabilities> = OnceCell::new();
pub static LAZY_LOG_PROBABILITIES: Lazy<LogProbabilities> = Lazy::new(|| {
    let async_fn = async {
        let textsynth = text_synth::engine();
        let continuation = NonEmptyString::new("dog".into()).unwrap();
        textsynth.log_probabilities("The quick brown fox jumps over the lazy ".into(), continuation)
            .await
            .expect("network error")
            .expect("api error")
    };

    futures::executor::block_on(async_fn)
});

pub fn initialize_log_probabilities(log_probabilities: LogProbabilities) {
    let _ = LOG_PROBABILITIES.set(log_probabilities);
}

pub fn get_log_probabilities() -> &'static LogProbabilities {
    LOG_PROBABILITIES.get().expect("log probabilities not initialized")
}

pub fn is_log_probabilities_initialized() -> bool {
    LOG_PROBABILITIES.get().is_some()
}
