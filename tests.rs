mod db_test;

#[test]
fn test_db_exist() {
    db_test::test_db_exist().unwrap();
}

#[test]
fn test_valid_query() {
    db_test::test_valid_query().unwrap();
}