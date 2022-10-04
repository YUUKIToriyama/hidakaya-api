pub fn open_db() -> Result<rusqlite::Connection, rusqlite::Error> {
    let file_path = "./hidakaya.db";
    let connection = rusqlite::Connection::open(&file_path).unwrap();
    println!("{}", connection.is_autocommit());
    Ok(connection)
}
