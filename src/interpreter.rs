use crate::ast::*;
use crate::error::CompilerError;
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<String, i64>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Stmt]) -> Result<(), CompilerError> {
        for stmt in program {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<i64>, CompilerError> {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval_expr(expr)?;
                self.env.insert(name.clone(), value);
            }
            Stmt::Assign(name, expr) => {
                let value = self.eval_expr(expr)?;
                if self.env.contains_key(name) {
                    self.env.insert(name.clone(), value);
                } else {
                    return Err(CompilerError::RuntimeError(format!("Undefined variable: {}", name)));
                }
            }
            Stmt::If(cond, then_block, else_block) => {
                if self.eval_expr(cond)? != 0 {
                    for stmt in then_block {
                        self.eval_stmt(stmt)?;
                    }
                } else {
                    for stmt in else_block {
                        self.eval_stmt(stmt)?;
                    }
                }
            }
            Stmt::While(cond, body) => {
                while self.eval_expr(cond)? != 0 {
                    for stmt in body {
                        self.eval_stmt(stmt)?;
                    }
                }
            }
            Stmt::DoWhile(body, cond) => {
                loop {
                    for stmt in body {
                        self.eval_stmt(stmt)?;
                    }
                    if self.eval_expr(cond)? == 0 {
                        break;
                    }
                }
            }
            Stmt::For(var, start, cond, step, body) => {
                let mut i = self.eval_expr(start)?;
                self.env.insert(var.clone(), i);
                while self.eval_expr(cond)? != 0 {
                    for stmt in body {
                        self.eval_stmt(stmt)?;
                    }
                    i = self.eval_expr(step)?;
                    self.env.insert(var.clone(), i);
                }
            }
            Stmt::FnDecl(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
            Stmt::Return(expr) => {
                return Ok(Some(self.eval_expr(expr)?));
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
            }
        }
        Ok(None)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<i64, CompilerError> {
        match expr {
            Expr::Number(n) => Ok(*n),
            Expr::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Expr::Variable(name) => self.env.get(name).cloned().ok_or_else(|| CompilerError::RuntimeError(format!("Undefined variable: {}", name))),
            Expr::Binary(lhs, op, rhs) => {
                let l = self.eval_expr(lhs)?;
                let r = self.eval_expr(rhs)?;
                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => Ok(l / r),
                    BinOp::Eq => Ok((l == r) as i64),
                    BinOp::Neq => Ok((l != r) as i64),
                    BinOp::Gt => Ok((l > r) as i64),
                    BinOp::Lt => Ok((l < r) as i64),
                }
            }
            Expr::Call(name, args) => {
                if let Some((params, body)) = self.functions.get(name) {
                    if args.len() != params.len() {
                        return Err(CompilerError::RuntimeError("Incorrect argument count".to_string()));
                    }
                    let mut new_env = self.env.clone();
                    for (param, arg) in params.iter().zip(args) {
                        let value = self.eval_expr(arg)?;
                        new_env.insert(param.clone(), value);
                    }
                    let mut new_interpreter = Interpreter {
                        env: new_env,
                        functions: self.functions.clone(),
                    };
                    for stmt in body {
                        if let Ok(Some(result)) = new_interpreter.eval_stmt(stmt) {
                            return Ok(result);
                        }
                    }
                    Ok(0)
                } else {
                    Err(CompilerError::RuntimeError(format!("Undefined function: {}", name)))
                }
            }
        }
    }
}