use std::{collections::BTreeSet, env::vars_os, fmt::Display, rc::Rc};

#[derive(Debug)]
pub enum Rule {
    Alpha,
    Beta,
}

/// Expressions represent logic statements. They're built reccursively with
/// [build_tree](Expression::build_tree) function, which is called by the
/// [parse](Expression::parse) function ~~after some preprocessing~~.
#[derive(Clone, Debug)]
pub enum Expression {
    Literal(char),
    No(SubExpression),
    And(SubExpression, SubExpression),
    Or(SubExpression, SubExpression),
}

type SubExpression = Rc<Expression>;

impl Expression {
    /*
    fn preprocess(input: &str) -> String {
        input.replace(" ", "")
        //.replace("(", "")
        //.replace(")", "")
    }
    */

    /// Takes a logic statement in Reverse Polish Notation and splits it into
    /// an iterator of characters, it then calls [build_tree](Expression::build_tree).
    pub fn parse(expression: &str) -> (Expression, BTreeSet<char>) {
        // let expression = Self::preprocess(expression);
        let mut symbols = expression.chars();
        Self::build_tree(&mut symbols)
    }

    /// Builds the syntax tree of a given logic statement reccursively.
    fn build_tree(symbols: &mut impl Iterator<Item = char>) -> (Expression, BTreeSet<char>) {
        let symbol = symbols.next().expect("Bad formula");
        let mut variables: BTreeSet<char> = BTreeSet::new();
        match symbol {
            '-' => {
                let (expr, vars) = Self::build_tree(symbols);
                variables.extend(&vars);
                (Expression::No(Rc::new(expr)), variables)
            }
            '&' | '|' => {
                let (left, vars_left) = Self::build_tree(symbols);
                let (right, vars_right) = Self::build_tree(symbols);
                let left = Rc::new(left);
                let right = Rc::new(right);
                variables.extend(&vars_left);
                variables.extend(&vars_right);
                let expr = if symbol == '&' {
                    Expression::And(left, right)
                } else {
                    Expression::Or(left, right)
                };
                (expr, variables)
            }
            _ => {
                variables.insert(symbol);
                (Expression::Literal(symbol), variables)
            }
        }
    }

    /// Determines whether an expression is an Alpha-Rule or a Beta-Rule.
    pub fn rule(&self) -> Option<Rule> {
        match self {
            Expression::Literal(_) | Expression::No(_) => None,
            Expression::And(_, _) => Some(Rule::Alpha),
            Expression::Or(_, _) => Some(Rule::Beta),
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Literal(l) => format!("{l}"),
            Expression::No(sub) => format!("-({})", sub.to_string()),
            Expression::And(left, right) => format!("({}) & ({})", left.to_string(), right.to_string()),
            Expression::Or(left, right) => format!("({}) | ({})", left.to_string(), right.to_string()),
        }
    }
}
