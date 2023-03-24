use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct PermanentError {
    pub source: Box<dyn Error>,
    pub message: Option<String>,
}

impl Display for PermanentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.source.as_ref(), formatter)
    }
}

impl Error for PermanentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
