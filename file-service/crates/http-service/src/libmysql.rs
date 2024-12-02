use mysql::{Pool,Row};
use std::error::Error;
use std::result::Result;


//todo!
pub trait DatabaseConnector {
    fn connect(&self) -> Result<Pool, Box<dyn Error>>;
    fn query(&self, query: &str) -> Result<Vec<Row>, Box<dyn Error>>;
}


pub struct MysqlConnector {
    url: String,
    pool: Option<Pool>,
}

impl MysqlConnector {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            pool: None,
        }
    }
}

impl DatabaseConnector for MysqlConnector {
    fn connect(&self) -> Result<Pool, Box<dyn Error>> {
        todo!()
    }

    fn query(&self, query: &str) -> Result<Vec<Row>, Box<dyn Error>> {
        todo!()
    }
}


#[test]
fn test_mysql() -> Result<(), Box<dyn Error>> {
    let mysql_connector = MysqlConnector::new("mysql://username:password@localhost:3306/database_name");
    let pool = mysql_connector.connect()?;
    let rows = mysql_connector.query("SELECT id, name FROM users")?;
    for row in rows {
        println!("{:?}", row);
    }
    Ok(())
}