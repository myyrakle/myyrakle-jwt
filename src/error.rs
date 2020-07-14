pub struct JWTError {
    message: String,
}

impl std::error::Error for JWTError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn type_id(&self, _: private::Internal) -> std::any::TypeId
    where
        Self: 'static,
    {
        std::any::TypeId::of::<Self>()
    }
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        None
    }
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        std::error.source()
    }
}

impl std::fmt::Display for JWTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
