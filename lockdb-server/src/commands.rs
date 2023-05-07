#[derive(Debug, Clone, Copy)]
pub enum Command {
    Exit,
    Create,
    Database,
    Table,
    Clear,
    Help,
}
#[derive(Debug, Clone)]
pub struct Commands <'a> {
    pub _c: Vec<Command>,
    pub _v: Vec<&'a str>
}



impl <'a> Commands <'a> {
    pub fn new() -> Self {
        Self { 
            _c: Vec::new(),
            _v: Vec::new()
        }
    }
}

impl <'a> FromIterator<&'a str> for Commands <'a> {
    
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut cmds: Vec<Command> = Vec::new();
        let mut vals = Vec::new();
        for i in iter {
            match i {
                "exit" | "quit" => {
                    cmds.push(Command::Exit);
                }
                "create" => {
                    cmds.push(Command::Create);
                }
                "table" => {
                    cmds.push(Command::Table);
                }
                "database" => {
                    cmds.push(Command::Database);
                }
                "clear" => {
                    cmds.push(Command::Clear);
                }
                "?" | "/?" | "help" => {
                    cmds.push(Command::Help);
                }
                _ => {
                    vals.push(i);
                }
            }
        }

        Self { _c: cmds, _v: vals }
    }
}