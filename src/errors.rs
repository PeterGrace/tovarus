use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum TovarusError {
    #[error("General Error: {0}")]
    General(String),
    #[error("Environment variable already exists.")]
    EnvVarAlreadyExists,
}
