use crate::{ast, x64};

pub fn codegen(node: ast::Expr) -> Vec<x64::Code> {
    gen_expr(node, Vec::new())
}

fn gen_expr(node: ast::Expr, mut lirs: Vec<x64::Code>) -> Vec<x64::Code> {
    match node {
        ast::Expr::Integer { value } => {
            lirs.push(x64::Code::PushInt128 { value });
        }
        ast::Expr::Add { lhs, rhs } => {
            lirs = gen_binary_operation('+', lhs, rhs, lirs);
        }
        ast::Expr::Sub { lhs, rhs } => {
            lirs = gen_binary_operation('+', lhs, rhs, lirs);
        }
    }

    lirs
}

fn gen_binary_operation(
    operator: char,
    lhs: Box<ast::Expr>,
    rhs: Box<ast::Expr>,
    lirs: Vec<x64::Code>,
) -> Vec<x64::Code> {
    let lirs = gen_expr(*lhs, lirs);
    let mut lirs = gen_expr(*rhs, lirs);
    lirs.push(x64::Code::PopRDI);
    lirs.push(x64::Code::PopRAX);
    match operator {
        '+' => {
            lirs.push(x64::Code::AddRDIToRAX);
        }
        '-' => {
            lirs.push(x64::Code::SubRDIFromRAX);
        }
        _ => unreachable!(),
    }
    lirs.push(x64::Code::PushRAX);

    lirs
}

#[cfg(test)]
mod codegen_tests {
    use super::*;

    #[test]
    fn generate_from_addition_test() {
        let node = ast::Expr::Add {
            lhs: Box::new(ast::Expr::Integer { value: 100 }),
            rhs: Box::new(ast::Expr::Integer { value: 200 }),
        };
        let codes = codegen(node);
        assert_eq!(6, codes.len());
    }
}
