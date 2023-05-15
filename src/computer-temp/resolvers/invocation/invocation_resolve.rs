use crate::computer::resolvers::{variable_reading, compose::compose};
use crate::public::compile_time::ast::types::{InvocationNode, ExpressionNode};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::error::type_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::Function;
use crate::public::value::value::{Value, ValueType};

use super::{build_in_function, lazy_expression, user_defined_function};

fn variable_invoke(
    func_name: &String,
    params: &Vec<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let func_value =
        variable_reading::resolve(func_name, scope)?;
    let result =
        function_invoke(func_value, params, scope)?;
    Ok(result)
}

fn function_invoke(
    function_value: Value,
    params: &Vec<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let invoke_result =
    match function_value {
        Value::LazyExpression(le) =>
            lazy_expression::invoke(le, scope)?,
        Value::Function(func_enum) => {
            match func_enum {
                Function::BuildIn(build_in_fn) => {
                    build_in_function::invoke(
                        build_in_fn.clone(),
                        params, scope,
                    )?
                },
                Function::UserDefined(user_defined_fn) => {
                    user_defined_function::invoke(
                        &user_defined_fn, 
                        params, scope,
                    )?
                },
            }
        },
        _ => {
            return Err(type_error(
                None,
                ValueType::Function,
                function_value.get_type()
            )?)
        }
    };
    Ok(invoke_result)
}

pub fn resolve(
    node: &InvocationNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let params =
        &node.params;

    let func_result =
    match node.caller.as_ref() {
        ASTNode::Variable(sub_node) => {
            variable_invoke(&sub_node.name, params, scope)?
        },
        ASTNode::Invocation(_) |
        ASTNode::ObjectReading(_) |
        ASTNode::ArrayElementReading(_) => {
            let function_value =
                compose::resolve(&node.caller, scope)?;
            function_invoke(function_value, params, scope)?
        },
        _ => {
            println!("Invalid callable target.");
            return Err(())
        }
    };
    Ok(func_result)
}