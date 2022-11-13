#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),

    #[error("Not found.")]
    NotFound,

    #[error("Something went wrong with tmux")]
    TmuxError,

    #[error("Permission Denied.")]
    PermissionDenied,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}
