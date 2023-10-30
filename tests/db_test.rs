extern crate rusqlite;
// use std::error::Error;

#[test]
fn test_database_exists() {
    use std::path::Path;

    let path = Path::new("book.db");
    assert!(path.exists());
}

// #[test]
// fn test_valid_query() -> Result<(), SqliteError> {
//     let db_name = "book.db";
//     let conn = connect_to_database(db_name)?;
//     let query_read = "SELECT * FROM books WHERE name=?1";
//     let input_read = "Educated";
//     let _result = read_data(&conn, query_read, input_read);  // If you're not using the result, you can prefix it with _
//     Ok(())
// }