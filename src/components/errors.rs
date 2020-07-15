pub struct JWTError {
    message: String,
}

impl JWTError {
    pub fn new(message: String) -> JWTError {
        JWTError { message }
    }
}

impl std::error::Error for JWTError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for JWTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JWT Error: {}", self.message)
    }
}

impl std::fmt::Debug for JWTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JWT Error: {}", self.message)
    }
}
