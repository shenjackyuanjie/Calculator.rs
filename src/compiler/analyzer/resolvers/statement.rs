use std::rc::Rc;

use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::ast_enum::{ASTNode, ASTVec};
use crate::public::compile_time::ast::types::{ExpressionNode, VariableNode};
use crate::public::compile_time::{keywords::Keywords, ast::types::StatementNode};
use crate::public::error::{import_error, syntax_error};
use crate::public::value::parens::Parens;

use super::{expression, statement_block};

fn statement_condition_resolve(
    tokens: &mut TokenVec
) -> Result<ExpressionNode, ()> {
    let first_index = 0;
    let mut sub_tokens = TokenVec::new();
    // sub condition tokens

    while first_index < tokens.len() {
        let current =
            tokens.pop_front().unwrap();
        //                         '{'
        if current == Token::Paren(Parens::LeftBrace) { break }
        sub_tokens.push_back(current);
    }
    Ok(expression::resolve(&mut sub_tokens)?)
}

pub fn resolve(
    keyword: Keywords,
    tokens: &mut TokenVec
) -> Result<StatementNode, ()> {
    // remove the keyword token
    tokens.pop_front();

    let statement_condition;
    let statement_body;
    // let mut params = ASTVec::new();

    match keyword {
        Keywords::Out => {
            let output_expression =
                expression::resolve(tokens)?;
            statement_condition = None;
            statement_body = vec![ASTNode::Expression(
                output_expression.into()
            )];
        },
        Keywords::For => {
            statement_condition = Some(statement_condition_resolve(tokens)?);
            statement_body = statement_block::resolve(tokens)?;
        },
        Keywords::If => {
            statement_condition = Some(statement_condition_resolve(tokens)?);
            statement_body = statement_block::resolve(tokens)?;
        },
        Keywords::Import => {
            if tokens.len() == 0 {
                return Err(import_error("module name missing")?)
            }
            statement_condition = None;

            let next_token =
                tokens.pop_front().unwrap();
            if let Token::Identi(module_name) = next_token {
                statement_body = vec![
                    ASTNode::Variable(
                        Rc::new(VariableNode {
                            name: module_name
                        })
                )]
            } else
            if let Token::String(module_path) = next_token {
                statement_body = vec![ASTNode::StringLiteral(module_path)]
            } else {
                return Err(import_error("invalid module name")?)
            }
        },

        Keywords::Break => {
            let return_expression =
                expression::resolve(tokens)?;
            statement_condition = None;
            statement_body = vec![ASTNode::Expression(
                return_expression.into()
            )];
        },
        Keywords::Continue => {
            statement_condition = None;
            statement_body = ASTVec::new();
        }, // Do nothing
        _ => {
            // example:
            // if 1 {new}
            let msg = format!("unexpected keyword '{}' in statement", keyword);
            return Err(syntax_error(&msg)?)
        }
    }

    Ok(StatementNode {
        keyword,
        condition: statement_condition,
        body: statement_body,
    })
}