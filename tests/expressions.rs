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

#[test]
fn test_equality_double_equals() {
    test("SELECT 26==26;", vec![LiteralValue::Boolean(true)]);
}

#[test]
fn test_equality_with_spaces() {
    test("SELECT 26 = 26;", vec![LiteralValue::Boolean(true)]);
}

#[test]
fn test_addition() {
    test("SELECT 5 + 6;", vec![LiteralValue::Integer(11)]);
}

#[test]
fn test_subtraction() {
    test("SELECT 5 - 6;", vec![LiteralValue::Integer(-1)]);
}

#[test]
fn test_multiple_additions() {
    test("SELECT 5 + 6 + 10 + 3 + 1;", vec![LiteralValue::Integer(25)]);
}

#[test]
fn test_multiple_subtractions() {
    test("SELECT 5 - 6 - 10 - 3 - 1;", vec![LiteralValue::Integer(-15)]);
}

#[test]
fn test_multiple_additions_and_subtractions() {
    test("SELECT 5 + 6 - 10 + 3 - 1;", vec![LiteralValue::Integer(3)]);
}

#[test]
fn test_unary_neg() {
    test("SELECT -5;", vec![LiteralValue::Integer(-5)]);
}

#[test]
fn test_paren_expr() {
    test("SELECT (5-6);", vec![LiteralValue::Integer(-1)]);
}

#[test]
fn test_neg_paren() {
    test("SELECT -(-6);", vec![LiteralValue::Integer(6)]);
}

#[test]
fn test_neg_paren_inner_expr() {
    test("SELECT -(3-44);", vec![LiteralValue::Integer(41)]);
}
