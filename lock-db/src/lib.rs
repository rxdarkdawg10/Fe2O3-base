#[derive(Debug)]
pub struct Database {
    pub in_use: bool,
    pub dbname: String,
    pub dbsize: usize,
    pub tables: Vec<Table>
}

#[derive(Debug)]
pub struct Table {
    pub tablename: String,
    pub tblsize: usize,
    pub columns: Vec<Columns>
}

#[derive(Debug)]
pub struct Columns {
    pub colname: String,
    pub coltype: Column,
    pub colsize: usize
}

#[derive(Debug, Clone, Copy)]
pub enum Column {
    String,
    Number,
    Binary
}

impl Database {
    pub fn new(dbname: String) -> Self {
        Self {
            in_use: true,
            dbname,
            dbsize: 0,
            tables: Vec::new()
        }
    }
}

impl Table {
    pub fn new(table_name: String) -> Self {
        Self {
            tablename: table_name,
            columns: Vec::new(),
            tblsize: 0,
        }
    }
}


 

impl Columns {
    pub fn new(colname: String, _ty: Column, size: usize) -> Self {
        Self { colname, coltype: _ty, colsize: size }
    }
}

impl std::fmt::Display for Column {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column::String => write!(f, "String"),
            Column::Number => write!(f, "Number"),
            Column::Binary => write!(f, "Binary"),
        }
    }
}

impl TryInto<Column> for String {
    type Error = String;
    
    fn try_into(self) -> Result<Column, Self::Error> {
        match self.as_str() {
            "number" => Ok(Column::Number),
            "string" => Ok(Column::String),
            "binary" => Ok(Column::Binary),
            _ => Err(String::from("Column Type Not supported"))
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
