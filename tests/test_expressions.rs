#![feature(int_uint)]
#![allow(unstable)]

extern crate rusql;

use rusql::{rusql_exec, Rusql, LiteralValue};

fn test(sql_str: &str, expected: Vec<LiteralValue>) {
    let mut db = Rusql::new();
    let result_table = rusql_exec(&mut db, sql_str, |_,_| {}).unwrap();

    let results = result_table.data.get(&1).unwrap();

    assert_eq!(&expected, results);
}

fn test_expect_ints(sql_str: &str, expected: Vec<int>) {
    let mut db = Rusql::new();
    let mut results: Vec<int> = Vec::new();
    let result_table = rusql_exec(&mut db, sql_str, |_,_| {}).unwrap();
    let result_row = result_table.data.get(&1).unwrap();

    for column in result_row.iter() {
        results.push(column.to_int());
    }

    assert_eq!(expected, results);
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
    test_expect_ints("SELECT 5 + 6;", vec![11]);
}

#[test]
fn test_subtraction() {
    test_expect_ints("SELECT 5 - 6;", vec![-1]);
}

#[test]
fn test_multiple_additions() {
    test_expect_ints("SELECT 5 + 6 + 10 + 3 + 1;", vec![25]);
}

#[test]
fn test_multiple_subtractions() {
    test_expect_ints("SELECT 5 - 6 - 10 - 3 - 1;", vec![-15]);
}

#[test]
fn test_multiple_additions_and_subtractions() {
    test_expect_ints("SELECT 5 + 6 - 10 + 3 - 1;", vec![3]);
}

#[test]
fn test_unary_neg() {
    test_expect_ints("SELECT -5;", vec![-5]);
}

#[test]
fn test_paren_expr() {
    test_expect_ints("SELECT (5-6);", vec![-1]);
}

#[test]
fn test_neg_paren() {
    test_expect_ints("SELECT -(-6);", vec![6]);
}

#[test]
fn test_neg_paren_inner_expr() {
    test_expect_ints("SELECT -(3-44);", vec![41]);
}

#[test]
fn test_multiplication() {
    test_expect_ints("SELECT 2*6;", vec![12]);
}

#[test]
fn test_division() {
    test_expect_ints("SELECT 15/3;", vec![5]);
}

#[test]
fn test_modulo() {
    test_expect_ints("SELECT 15%6;", vec![3]);
}

#[test]
fn test_not_equals() {
    test_expect_ints("SELECT 5!=4, 5<>4, 5!=5;", vec![1, 1, 0]);
}

#[test]
fn test_and() {
    test_expect_ints("SELECT 0 AND 0, 0 AND 1, 1 AND 0, 1 AND 1;", vec![0, 0, 0, 1]);
}

#[test]
fn test_or() {
    test_expect_ints("SELECT 0 OR 0, 0 OR 1, 1 OR 0, 1 OR 1;", vec![0, 1, 1, 1]);
}

#[test]
fn test_not() {
    test_expect_ints("SELECT NOT 1, NOT 0, NOT (5 == 5);", vec![0, 1, 0]);
}

#[test]
fn test_multiple_boolean_ops() {
    test_expect_ints("SELECT 3=3 AND 4=4, (3=3) AND (4=4);", vec![1, 1]);
}

#[test]
fn test_less_than() {
    test_expect_ints("SELECT 3<4, 3<3, 3<2;", vec![1, 0, 0]);
}

#[test]
fn test_less_than_or_eq() {
    test_expect_ints("SELECT 3<=4, 3<=3, 3<=2;", vec![1, 1, 0]);
}

#[test]
fn test_greater_than() {
    test_expect_ints("SELECT 3>4, 3>3, 3>2;", vec![0, 0, 1]);
}

#[test]
fn test_greater_than_or_eq() {
    test_expect_ints("SELECT 3>=4, 3>=3, 3>=2;", vec![0, 1, 1]);
}

#[test]
fn test_bit_and() {
    test_expect_ints("SELECT 6 & 3;", vec![2]);
}

#[test]
fn test_bit_or() {
    test_expect_ints("SELECT 6 | 3;", vec![7]);
}

#[test]
fn test_right_shift() {
    test_expect_ints("SELECT 6 >> 1;", vec![3]);
}

#[test]
fn test_left_shift() {
    test_expect_ints("SELECT 6 << 1;", vec![12]);
}

#[test]
fn test_bit_neg() {
    test_expect_ints("SELECT ~7;", vec![-8]);
}

#[test]
fn test_mult_div_associativity() {
    test_expect_ints("SELECT 9/3*3;", vec![9]);
}
