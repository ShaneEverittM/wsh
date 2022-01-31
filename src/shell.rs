use std::ffi::OsStr;
use std::process::ExitStatus;
use std::{
    env,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
    process::Command,
    str::SplitWhitespace,
};

use crate::{
    error::WshError,
    input::{Builtin, Input},
    runtime::OrExit,
};

pub struct Shell {
    home_dir: PathBuf,
    last_command_status: Option<ExitStatus>,
}

impl Shell {
    pub fn new() -> Result<Self, WshError> {
        let home_dir = directories::BaseDirs::new()
            .ok_or_else(|| WshError::Environment("Cannot find home directory".into()))?
            .home_dir()
            .to_path_buf();

        Ok(Self {
            home_dir,
            last_command_status: None,
        })
    }

    fn teardown(self, result: Result<(), WshError>) -> ! {
        // Cleanup goes here when necessary

        result.or_exit();
        std::process::exit(exitcode::OK)
    }

    pub fn start(mut self) {
        let res = self.run();

        self.teardown(res)
    }

    fn run(&mut self) -> Result<(), WshError> {
        loop {
            Self::prompt()?;

            let mut input = String::new();
            if let Ok(0) = stdin().read_line(&mut input) {
                return Ok(());
            }

            let mut parts = input.trim().split_whitespace();
            let command = match parts.next() {
                None => continue,
                Some(command) => command,
            };

            let args = parts;

            match Input::from_str(command) {
                Input::Builtin(builtin) => {
                    self.handle_builtin(builtin, args)?;
                }
                Input::Command(command) => {
                    self.last_command_status = Some(Self::run_command(command.as_ref(), args)?);
                }
            }
        }
    }

    fn run_command<S, I>(command: S, args: I) -> Result<ExitStatus, WshError>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        let spawn_result = Command::new(command).args(args).spawn();

        match spawn_result {
            Ok(mut child) => child.wait().map_err(WshError::from),
            Err(e) => Err(WshError::from(e)),
        }
    }

    fn prompt() -> Result<(), WshError> {
        print!("> ");
        stdout().flush().map_err(WshError::from)
    }
}

// Built-ins
impl Shell {
    fn handle_builtin(&self, builtin: Builtin, args: SplitWhitespace) -> Result<(), WshError> {
        match builtin {
            Builtin::Cd => {
                self.cd(args);
                Ok(())
            }

            // Mildly clunky since this isn't really an error, but it abnormal I guess
            Builtin::Exit => Err(WshError::Exit("Exit requested".to_owned())),
        }
    }

    fn cd(&self, args: SplitWhitespace) {
        let new_dir = args
            .peekable()
            .peek()
            .map(|&s| Path::new(s))
            .unwrap_or_else(|| self.home_dir.as_path());

        if let Err(e) = env::set_current_dir(&new_dir) {
            eprintln!("{}", e);
        }
    }
}
