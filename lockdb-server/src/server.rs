use std::path::PathBuf;

use lock_db::Database;

#[derive(Debug)]
pub struct Server {
    pub _port: u16,
    pub filepath: PathBuf,
    pub databases: Vec<Database>
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { 
            _port: port,
            databases: Vec::new(),
            filepath: PathBuf::new()
        }
    }

    pub fn get_schema_as_json(&self) -> Result<String, ()> {
        let mut result = String::from("Schema: {");

        result.push_str("Databases: [");
        for d in 0..self.databases.len() {
            result.push_str("{");
            //dbname
            result.push_str("dbname: \"");
            result.push_str(self.databases[d].dbname.as_str());
            result.push_str("\",");

            //dbsize
            result.push_str("dbsize: ");
            result.push_str(self.databases[d].dbsize.to_string().as_str());
            result.push_str(",");

            //tables
            result.push_str("Tables: [");
            for t in 0..self.databases[d].tables.len() {
                result.push_str("{");
                //dbname
                result.push_str("tblname: \"");
                result.push_str(self.databases[d].tables[t].tablename.as_str());
                result.push_str("\",");
                //dbname
                result.push_str("tblsize: \"");
                result.push_str(self.databases[d].tables[t].tblsize.to_string().as_str());
                result.push_str("\",");

                result.push_str("},");
            }
            result.push_str("]");

            result.push_str("},");
        }
        result.push_str("]");

        result.push_str("}\0");
        Ok(result)
    }
}

#[derive(Debug)]
pub struct Configuration;