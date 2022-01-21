use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::OnceCell;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static LAST_ERROR: OnceCell<Arc<dotenv::Error>> = OnceCell::new();
const ORDERING: Ordering = Ordering::SeqCst;

pub fn initialize() {
    if let Some(error) = LAST_ERROR.get() {
        panic!("failed to initialize dotenv: {error}");
    } else if !INITIALIZED.load(ORDERING) {
        let result = match dotenv::dotenv().map_err(Arc::new) {
            Ok(_) => {
                INITIALIZED.store(true, ORDERING);
                Ok(())
            },
            Err(error) => {
                let _ = LAST_ERROR.set(Arc::clone(&error));
                Err(error)
            }
        };

        result.expect("failed to initialize dotenv")
    }
}
