use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UntaggedResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for UntaggedResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => UntaggedResult::Ok(value),
            Err(error) => UntaggedResult::Err(error),
        }
    }
}

impl<T, E> From<UntaggedResult<T, E>> for Result<T, E> {
    fn from(untagged_result: UntaggedResult<T, E>) -> Self {
        match untagged_result {
            UntaggedResult::Ok(value) => Ok(value),
            UntaggedResult::Err(error) => Err(error),
        }
    }
}
