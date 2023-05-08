#[derive(Debug, Clone, Copy)]
pub enum Command {
    Exit,
    Create,
    Database,
    Table,
    Clear,
    Help,
    Use,
    Add,
    Column,
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
            "use" => Ok(Command::Use),
            "add" => Ok(Command::Add),
            "column" => Ok(Command::Column),
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
