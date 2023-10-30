use rusqlite::{params, Connection, Result};

pub fn connect_to_database(db: &str) -> Result<Connection> {
    let conn = Connection::open(db);
    println!("Connection to database completed");
    conn
}

pub fn create_table(conn: &Connection, query_creation: &str) -> Result<()> {
    conn.execute(query_creation, [])?;
    println!("Table created successfully");
    Ok(())
}

pub fn insert_data(
    conn: &Connection,
    sql_insertion: &str,
    i1: &str,
    i2: i64,
    i3: &str,
) -> Result<()> {
    conn.execute(sql_insertion, params![i1, i2, i3])?;
    println!("Insertion done successfully");
    Ok(())
}

pub fn read_data(conn: &Connection, sql_read: &str, input_read: &str) -> Result<()> {
    let mut stmt = conn.prepare(sql_read)?;
    let rows = stmt.query_map(params![input_read], |row| {
        let name: String = row.get(1)?;
        let stock: i64 = row.get(2)?;
        let comment: String = row.get(3)?;
        Ok((name, stock, comment))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

pub fn update_data(conn: &Connection, query_update: &str, stock: i64, name: &str) -> Result<()> {
    conn.execute(query_update, params![stock, name])?;
    println!("Update done successfully");
    Ok(())
}

pub fn delete_data(conn: &Connection, sql_delete: &str, name2: &str) -> Result<()> {
    conn.execute(sql_delete, params![name2])?;
    println!("Deletion done successfully");
    Ok(())
}

pub fn count_book_by_stock(conn: &Connection, stock: i64) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM books WHERE stock=?",
        params![stock],
        |row| row.get(0),
    )
}

pub fn fetch_books_ordered_by_name(conn: &Connection) -> Result<Vec<(i64, String, i64, String)>> {
    let mut stmt = conn.prepare("SELECT * FROM books ORDER BY name")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;

    Ok(rows.map(|row| row.unwrap()).collect())
}


// main test part
#[test]
fn test_connect_to_database() {
    let conn = connect_to_database("test.db");
    assert!(conn.is_ok());
}

#[test]
fn test_create_table() {
    let conn = connect_to_database("test.db").unwrap();
    let query_creation = "CREATE TABLE IF NOT EXISTS books (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        stock INTEGER NOT NULL,
        comment TEXT
    )";
    let result = create_table(&conn, query_creation);
    assert!(result.is_ok());
}