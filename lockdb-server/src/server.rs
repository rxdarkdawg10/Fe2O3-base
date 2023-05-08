use lock_db::Database;

#[derive(Debug)]
pub struct Server {
    pub _port: u16,
    pub databases: Vec<Database>
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { 
            _port: port,
            databases: Vec::new()
        }
    }
}