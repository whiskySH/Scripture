use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("JNI error: {0}")]
    Jni(String),
}

pub type ModResult<T> = Result<T, ModError>;