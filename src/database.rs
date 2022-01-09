use postgres::{Client, NoTls, Error};

pub fn create_database() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:password@localhost:5434", NoTls)?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS stock (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            ticker          VARCHAR NOT NULL
            )
    ")?;
    Ok(())
}