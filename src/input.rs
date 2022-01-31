pub enum Builtin {
    Cd,
    Exit,
}

pub enum Input {
    Builtin(Builtin),
    Command(String),
}

impl Input {
    pub fn from_str(s: &str) -> Input {
        match s {
            "cd" => Input::Builtin(Builtin::Cd),
            "exit" => Input::Builtin(Builtin::Exit),
            _ => Input::Command(s.to_owned()),
        }
    }
}
