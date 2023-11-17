use log::{log, Level};

use super::Context;

pub enum ErrorCode {
    Undefined,
    File,
    Range,
    Internal,
    Null,
    Read,
    Seek,
    Write,
    UnknownExtension,
    ColorspaceCheck,
    AlreadyDefined,
    BadSignature,
    CorruptionDetected,
    NotSuitable,
}

impl ErrorCode {
    pub fn unwrap(&self) -> &'static str {
        match self {
            ErrorCode::Undefined => "Undefined error",
            ErrorCode::File => "File system error",
            ErrorCode::Range => "Value out of range",
            ErrorCode::Internal => "Internal error",
            ErrorCode::Null => "Value was null",
            ErrorCode::Read => "IO read error",
            ErrorCode::Seek => "IO seek error",
            ErrorCode::Write => "IO write error",
            ErrorCode::UnknownExtension => "Unknown extension type",
            ErrorCode::ColorspaceCheck => "Invalid color space",
            ErrorCode::AlreadyDefined => "Value already defined",
            ErrorCode::BadSignature => "Object has bad signature",
            ErrorCode::CorruptionDetected => "Corruption detected",
            ErrorCode::NotSuitable => "Value not suitable",
        }
    }
}

pub type ErrorHandlerLogFunction =
    fn(context_id: &Context, level: Level, error_code: ErrorCode, text: &'static str);

pub fn default_error_handler_log_function(
    _context_id: &Context,
    level: Level,
    error_code: Option<ErrorCode>,
    text: &'static str,
) {
    if let Some(error_code) = error_code {
        log!(level, "[{}] => {}", error_code.unwrap(), text)
    }
}
