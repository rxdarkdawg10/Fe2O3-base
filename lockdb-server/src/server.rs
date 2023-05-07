#[derive(Debug)]
pub struct Server {
    pub _port: u16,

}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { _port: port }
    }
}