#[derive(Debug)]
pub struct Database {
    dbname: String,
    pub tables: Vec<Table>
}

#[derive(Debug)]
pub struct Table {
    tablename: String,
    columns: Vec<Columns>
}

#[derive(Debug)]
pub struct Columns;

impl Database {
    pub fn new(dbname: String) -> Self {
        Self {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
