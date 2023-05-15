use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::type_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, Overload, ValueType};

use super::super::expression;

fn index_resolve(
    expression_node: &ExpressionNode,
    scope: &mut Scope
) -> Result<usize, ()> {
    let index_value =
        expression::resolve(expression_node, scope)?;
    if let Value::Number(num) = index_value {
        if num < Number::Int(0) {
            println!("Index of an array should not be less than ZERO.");
            return Err(())
        }
        Ok(num.int_value() as usize)
    } else {
        // index type error
        Err(type_error(
            Some("array index"),
            ValueType::Number,
            index_value.get_type(),
        )?)
    }
}

fn check_outof_range(
    index: usize,
    len: usize,
) -> Result<(), ()> {
    if index >= len {
        println!("Array reading out of range, expected index < {}, found {}.", len, index);
        Err(())
    } else {
        Ok(())
    }
}

// --- --- --- --- --- ---

pub fn assign(
    array_value: Value,
    index_node: &ExpressionNode,
    value: Value,
    scope: &mut Scope,
) -> Result<(), ()> {
    let index_value =
        index_resolve(index_node, scope)?;
    if let Value::Array(arr_ref) = array_value {
        // array writing
        let mut arr =
            arr_ref.as_ref().borrow_mut();
        check_outof_range(index_value, arr.len())?;
        arr[index_value] = value;
    } else
    if let Value::String(str_ref) = array_value {
        // string writing
        let mut str =
            str_ref.as_ref().borrow_mut();

        let Value::String(target) = value else {
            return Err(type_error(
                Some("string assignment"),
                ValueType::String,
                value.get_type(),
            )?)
        };
        check_outof_range(index_value, str.len())?;
        let char_str =
            &target.as_ref().borrow();
        str.replace_range(index_value..index_value+1, char_str);
    } else {
        println!("Invalid array reading.");
        return Err(())
    }
    Ok(())
}

pub fn resolve(
    array_value: Value,
    index_node: &ExpressionNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let index_value =
        index_resolve(index_node, scope)?;

    if let Value::Array(arr_ref) = array_value {
        let arr =
            arr_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, arr.len())?;
        Ok(arr[index_value].clone())
    } else
    if let Value::String(str_ref) = array_value {
        let str =
            str_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, str.len())?;
        let slice = &str[index_value..index_value+1];
        Ok(Value::create(slice.to_string()))
    } else {
        println!("Invalid indexing.");
        Err(())
    }
}