#[derive(Debug, Clone, Copy)]
pub enum Command {
    Exit,
    Create,
    Database,
    Table,
    Clear,
    Help,
    Space,
}
#[derive(Debug, Clone)]
pub struct Commands {
    pub _c: Vec<Command>,
    pub _v: Vec<String>
}

impl Command {
    pub fn new(val: String) -> Result<Command, String> {
        match val.as_str() {
            "exit" | "quit" => Ok(Command::Exit),
            "create" => Ok(Command::Create),
            "table" => Ok(Command::Table),
            "database" => Ok(Command::Database),
            "clear" => Ok(Command::Clear),
            "?" | "/?" | "help" => Ok(Command::Help),
            _ => Err(val)
        }
    }
}


impl Commands {
    pub fn new() -> Self {
        Self { 
            _c: Vec::new(),
            _v: Vec::new()
        }
    }
}

impl FromIterator<String> for Commands {
    
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut cmds: Vec<Command> = Vec::new();
        let mut vals = Vec::new();
        for i in iter {
            match i.as_str() {
                "exit" | "quit" => {
                    cmds.push(Command::Exit);
                }
                // "create" => {
                //     cmds.push(Command::Create);
                // }
                // "table" => {
                //     cmds.push(Command::Table);
                // }
                // "database" => {
                //     cmds.push(Command::Database);
                // }
                // "clear" => {
                //     cmds.push(Command::Clear);
                // }
                // "?" | "/?" | "help" => {
                //     cmds.push(Command::Help);
                // }
                _ => {
                    vals.push(i);
                }
            }
        }

        Self { _c: cmds, _v: vals }
    }
}