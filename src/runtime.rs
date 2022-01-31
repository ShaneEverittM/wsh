use crate::error::WshError;

pub trait OrExit<T> {
    fn or_exit(self) -> T;
}

// Used for failure to even make the shell, so we don't need the shell instance for cleanup
impl<T> OrExit<T> for Result<T, WshError> {
    fn or_exit(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(exitcode::ExitCode::from(e))
            }
        }
    }
}
