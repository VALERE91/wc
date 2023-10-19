use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum FileStatsError {
    NotFound,
    PermissionDenied,
    Other{msg: String}
}

impl Display for FileStatsError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatsError::NotFound => write!(f, "File not found"),
            FileStatsError::PermissionDenied => write!(f, "Access to file denied"),
            FileStatsError::Other { msg } => write!(f, "{}", msg)
        }
    }
}