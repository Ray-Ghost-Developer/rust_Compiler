use crate::ast::*;
use crate::error::CompilerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Void,
}

pub struct TypeChecker {
    env: HashMap<String, Type>,
    functions: HashMap<String, (Vec<Type>, Type)>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, program: &[Stmt]) -> Result<(), CompilerError> {
        for stmt in program {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Let(name, expr) => {
                let t = self.check_expr(expr)?;
                self.env.insert(name.clone(), t);
            }
            Stmt::Assign(name, expr) => {
                let t = self.check_expr(expr)?;
                if let Some(var_type) = self.env.get(name) {
                    if *var_type != t {
                        return Err(CompilerError::TypeError(format!("Type mismatch in assignment to {}", name)));
                    }
                } else {
                    return Err(CompilerError::TypeError(format!("Undeclared variable: {}", name)));
                }
            }
            Stmt::If(cond, then_block, else_block) => {
                let cond_type = self.check_expr(cond)?;
                if cond_type != Type::Bool {
                    return Err(CompilerError::TypeError("Condition in 'if' must be a boolean".to_string()));
                }
                for stmt in then_block {
                    self.check_stmt(stmt)?;
                }
                for stmt in else_block {
                    self.check_stmt(stmt)?;
                }
            }
            Stmt::While(cond, body) | Stmt::DoWhile(body, cond) => {
                let cond_type = self.check_expr(cond)?;
                if cond_type != Type::Bool {
                    return Err(CompilerError::TypeError("Condition in loop must be a boolean".to_string()));
                }
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
            }
            Stmt::For(var, start, cond, step, body) => {
                let t_start = self.check_expr(start)?;
                let t_cond = self.check_expr(cond)?;
                let t_step = self.check_expr(step)?;
                if t_start != Type::Int || t_cond != Type::Bool || t_step != Type::Int {
                    return Err(CompilerError::TypeError("Invalid types in 'for' loop".to_string()));
                }
                self.env.insert(var.clone(), Type::Int);
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
            }
            Stmt::FnDecl(name, params, body) => {
                let param_types = vec![Type::Int; params.len()];
                self.functions.insert(name.clone(), (param_types.clone(), Type::Int));
                for (i, param) in params.iter().enumerate() {
                    self.env.insert(param.clone(), param_types[i].clone());
                }
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
            }
            Stmt::Return(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
            }
        }
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, CompilerError> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Variable(name) => self.env.get(name).cloned().ok_or_else(|| CompilerError::TypeError(format!("Undeclared variable: {}", name))),
            Expr::Binary(lhs, op, rhs) => {
                let lt = self.check_expr(lhs)?;
                let rt = self.check_expr(rhs)?;
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                        if lt == Type::Int && rt == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err(CompilerError::TypeError("Operands must be integers".to_string()))
                        }
                    }
                    BinOp::Eq | BinOp::Neq | BinOp::Gt | BinOp::Lt => {
                        if lt == rt {
                            Ok(Type::Bool)
                        } else {
                            Err(CompilerError::TypeError("Operands must be of the same type".to_string()))
                        }
                    }
                }
            }
            Expr::Call(name, args) => {
                if let Some((param_types, return_type)) = self.functions.get(name) {
                    if args.len() != param_types.len() {
                        return Err(CompilerError::TypeError(format!("Incorrect number of arguments in call to {}", name)));
                    }
                    for (arg, expected) in args.iter().zip(param_types) {
                        let arg_type = self.check_expr(arg)?;
                        if arg_type != *expected {
                            return Err(CompilerError::TypeError("Argument type mismatch".to_string()));
                        }
                    }
                    Ok(return_type.clone())
                } else {
                    Err(CompilerError::TypeError(format!("Undefined function: {}", name)))
                }
            }
        }
    }
}
