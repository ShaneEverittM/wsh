use runtime::OrExit;
use shell::Shell;

mod error;
mod input;
mod runtime;
mod shell;

fn main() {
    Shell::new().or_exit().start()
}
