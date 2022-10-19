use std::rc::Rc;

#[derive(Debug)]
pub enum Rule {
    Alpha,
    Beta,
}

/// Expressions represent logic statements. They're built reccursively with
/// [build_tree](Expression::build_tree) function, which is called by the
/// [parse](Expression::parse) function ~~after some preprocessing~~.
#[derive(Debug)]
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
    pub fn parse(expression: &str) -> Expression {
        // let expression = Self::preprocess(expression);
        let mut symbols = expression.chars();
        Self::build_tree(&mut symbols)
    }

    /// Builds the syntax tree of a given logic statement reccursively.
    fn build_tree(symbols: &mut impl Iterator<Item = char>) -> Expression {
        let symbol = symbols.next().expect("Bad formula");
        match symbol {
            '-' => Expression::No(Rc::new(Self::build_tree(symbols))),
            '&' | '|' => {
                let left = Rc::new(Self::build_tree(symbols));
                let right = Rc::new(Self::build_tree(symbols));
                if symbol == '&' {
                    Expression::And(left, right)
                } else {
                    Expression::Or(left, right)
                }
            }
            _ => Expression::Literal(symbol),
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
