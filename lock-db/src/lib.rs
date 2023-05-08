#[derive(Debug)]
pub struct Database {
    pub in_use: bool,
    pub dbname: String,
    pub tables: Vec<Table>
}

#[derive(Debug)]
pub struct Table {
    pub tablename: String,
    pub columns: Vec<Columns>
}

#[derive(Debug)]
pub struct Columns {
    pub colname: String,
    pub coltype: Column,
    pub colsize: u64
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
            tables: Vec::new()
        }
    }
}

impl Table {
    pub fn new(table_name: String) -> Self {
        Self {
            tablename: table_name,
            columns: Vec::new()
        }
    }
}


 

impl Columns {
    pub fn new(colname: String, _ty: Column, size: u64) -> Self {
        Self { colname, coltype: _ty, colsize: size }
    }
}


impl TryInto<Column> for String {
    type Error = ();
    
    fn try_into(self) -> Result<Column, Self::Error> {
        match self.as_str() {
            "number" => Ok(Column::Number),
            "string" => Ok(Column::String),
            "binary" => Ok(Column::Binary),
            _ => Err(())
        }
    }
    // type Error = ();

    // fn try_into(self) -> Result<Column, Self::Error> {
    //         match self.as_str() {
    //         "number" => {
    //             Ok(Column::Number)
    //         }
    //     }

    //     Err(())
    // }

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
