use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct PermanentError {
    pub source: Box<dyn Error>,
    pub message: Option<String>,
}

impl Display for PermanentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let source_message = format!("{}", self.source.as_ref());
        let exception_message = self
            .message
            .as_ref()
            .map(|message| format!(": {message}"))
            .unwrap_or("".to_string());

        write!(
            formatter,
            "{}{}",
            source_message.as_str(),
            exception_message.as_str()
        )
    }
}

impl Error for PermanentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
