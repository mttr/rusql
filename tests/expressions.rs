extern crate rusql;

use rusql::{rusql_exec, Rusql, LiteralValue};

fn test(sql_str: &str, expected: Vec<LiteralValue>) {
    let mut db = Rusql::new();
    let result_table = rusql_exec(&mut db, sql_str, |_,_| {}).unwrap();

    let results = result_table.data.get(&0).unwrap();

    assert_eq!(&expected, results);
}

#[test]
fn test_literal_values() {
    test("SELECT 26, \"Foo\";",
         vec![LiteralValue::Integer(26), LiteralValue::Text("Foo".to_string())]);
}

#[test]
fn test_equality() {
    test("SELECT 26=26;", vec![LiteralValue::Boolean(true)]);
    test("SELECT 26=27;", vec![LiteralValue::Boolean(false)]);
}
