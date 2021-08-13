use crate::{
    connstring::ConnectionString, db::ThreadedDatabase, Client, ClientOptions, ThreadedClient,
};

/// A basic r2d2 connection manager for this driver.
///
/// - returns a Database object matching the provided database name, not a Client
/// - takes a parsed connection string and client options
#[derive(Debug)]
pub struct MongoConnectionManager {
    conn_str: ConnectionString,
    db_name: String,
    client_options: Option<ClientOptions>,
}

impl MongoConnectionManager {
    pub fn new<S, CO>(connection_str: ConnectionString, db_name: S, client_options: CO) -> Self
    where
        S: Into<String>,
        CO: Into<Option<ClientOptions>>,
    {
        Self {
            conn_str: connection_str,
            db_name: db_name.into(),
            client_options: client_options.into(),
        }
    }
}

impl r2d2::ManageConnection for MongoConnectionManager {
    type Connection = crate::db::Database;
    type Error = crate::error::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::with_config(self.conn_str.clone(), self.client_options.clone(), None)?;

        // Try to acquire a stream to establish a connection. If we can't, the connection can't be used.
        client.acquire_stream(client.read_preference.clone())?;

        Ok(client.db(&self.db_name))
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.version()?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
