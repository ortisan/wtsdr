use std::error::Error;
use std::sync::Arc;

pub type ResultApp<T> = Result<T, Arc<dyn Error>>;
