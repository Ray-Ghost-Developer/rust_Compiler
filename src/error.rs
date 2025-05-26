#[allow(dead_code)]
#[derive(Debug)]
pub enum CompilerError {
    SyntaxError(String),
    TypeError(String),
    RuntimeError(String),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
            CompilerError::TypeError(msg) => write!(f, "Type error: {}", msg),
            CompilerError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for CompilerError {}