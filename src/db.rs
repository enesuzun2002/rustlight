use std::{collections::HashMap, path::PathBuf};

use rusqlite::{Connection, Error, Result as SqliteResult};

pub fn retrieve_installed_apps_from_cache() -> SqliteResult<HashMap<String, PathBuf>> {
    let conn = match sql_connection() {
        Ok(value) => value,
        Err(value) => return Err(value.unwrap_err().into()),
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS apps (
            name TEXT PRIMARY KEY,
            path TEXT NOT NULL
        )",
        [],
    ) {
        Ok(_) => {
            println!("Statement executed successfully");
        }
        Err(error) => {
            println!("Error: {}", error);
            // Handle the error case here
            return Err(error.into()); // or handle the error accordingly
        }
    };

    let mut installed_apps: HashMap<String, PathBuf> = HashMap::new();

    let mut stmt = match conn.prepare("SELECT name, path FROM apps") {
        Ok(statement) => statement,
        Err(error) => {
            println!("Error: {}", error);
            return Err(error.into()); // or handle the error accordingly
        }
    };
    let rows = match stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            PathBuf::from(row.get::<_, String>(1)?),
        ))
    }) {
        Ok(mapped_rows) => mapped_rows,
        Err(error) => {
            println!("Error: {}", error);
            return Err(error.into()); // or handle the error accordingly
        }
    };

    for row in rows {
        let (name, path) = row?;
        installed_apps.insert(name, path);
    }

    Ok(installed_apps)
}

pub fn save_installed_apps_to_cache(installed_apps: &HashMap<String, PathBuf>) -> SqliteResult<()> {
    let conn = match sql_connection() {
        Ok(value) => value,
        Err(value) => return value,
    };
    match conn.execute("DELETE FROM apps", []) {
        Ok(deleted) => println!("{} rows were deleted", deleted),
        Err(err) => println!("delete failed: {}", err),
    };

    let mut stmt = conn.prepare("INSERT INTO apps (name, path) VALUES (?, ?)")?;
    for (name, path) in installed_apps.iter() {
        stmt.execute([name, path.to_string_lossy().as_ref()])?;
    }

    Ok(())
}

pub fn sql_connection() -> Result<Connection, Result<(), Error>> {
    let conn: Connection = match Connection::open("rustlight.db") {
        Ok(connection) => {
            println!("Connection successful");
            connection
        }
        Err(error) => {
            println!("Error: {}", error);
            return Err(Err(error.into()));
        }
    };
    Ok(conn)
}
