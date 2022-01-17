use postgres::{Client, NoTls, Error};

pub struct Database {
    conn: Client
}

impl Database {

    pub fn new(configuration: String) -> Result<Database, Error> {
        let connection = Client::connect(&configuration, NoTls);
        match connection {
            Ok(result) => {
                let database = Database {
                    conn: result
                };
                database.conn.batch_execute("
                    CREATE TABLE IF NOT EXISTS stock (
                        ticker          SERIAL PRIMARY KEY,
                        name            VARCHAR NOT NULL,
                    )
                ");
                return Ok(database);
            },
            Err(e) => return Error(e),
        }
    }

    pub async fn add_ticker(&mut self, ticker: String) -> Result<(), Error> {
        self.conn.execute("INSERT INTO stocks (ticker) VALUES ($1)", &[&ticker]);
        Ok(())
    }

    pub async fn remove_ticker(&mut self, ticker: String) -> Result<(), Error> {
        self.conn.execute("DELETE FROM stocks WHERE (ticker) IS ($1)", &[&ticker]);
        Ok(())
    }
}