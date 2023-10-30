extern crate sqlite_rust_project;

use rusqlite::{params, Connection, Result};
use sqlite_rust_project::{connect_to_database,create_table,insert_data,read_data,update_data,delete_data,count_book_by_stock,fetch_books_ordered_by_name};

fn main() -> Result<()> {
    let db_name = "book.db";
    let conn = connect_to_database(db_name)?;

    // Create 
    let query_creation = "
        CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            stock INTEGER NOT NULL,
            comment TEXT
        )
    ";
    create_table(&conn, query_creation)?;

    let query_insertion = "INSERT INTO books (name, stock, comment) VALUES (?1, ?2, ?3)";
    insert_data(
        &conn,
        query_insertion,
        "Red Chamber",
        30,
        "Uncopiable Love Story",
    )?;
    insert_data(
        &conn,
        query_insertion,
        "Educated",
        35,
        "Meaning of Education",
    )?;

    // Read
    let query_read = "SELECT * FROM books WHERE name=?1";
    let input_read = "Red Chamber";
    read_data(&conn, query_read, input_read)?;

    // Update
    let query_update = "UPDATE books SET stock=?1 WHERE name=?2";
    let stock = 25;
    let name = "Red Chamber";
    update_data(&conn, query_update, stock, name)?;

    println!("{:?}", count_book_by_stock(&conn, 35)?);
    for row in fetch_books_ordered_by_name(&conn)? {
        println!("{:?}", row);
    }

    // Delete
    let query_deletion = "DELETE FROM books WHERE name=?1";
    let name2 = "Red Chamber";
    let name3 = "Educated";
    delete_data(&conn, query_deletion, name2)?;
    delete_data(&conn, query_deletion, name3)?;

    Ok(())
}
