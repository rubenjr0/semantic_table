use std::{collections::BTreeSet, rc::Rc};

use super::Expression;

#[derive(Debug)]
pub struct SemanticTable {
    expressions: Vec<Expression>,
    variables: BTreeSet<char>,
}

impl SemanticTable {
    pub fn parse_set(expressions: &Vec<&str>) -> SemanticTable {
        let expressions_and_variables = expressions
            .iter()
            .map(|expr| Expression::parse(expr))
            .collect::<Vec<_>>();
        let mut expressions = vec![];
        let mut variables = BTreeSet::new();
        for (expr, var) in expressions_and_variables {
            expressions.push(expr);
            variables.extend(&var);
        }
        SemanticTable {
            expressions,
            variables,
        }
    }

    pub fn expressions(&self) -> &Vec<Expression> {
        &self.expressions
    }

    pub fn variables(self) -> BTreeSet<char> {
        self.variables
    }

    pub fn models(&self, expr: Expression) -> bool {
        let mut omega_check = self.expressions.clone();
        omega_check.push(Expression::No(Rc::new(expr)));
        false
    }
}
