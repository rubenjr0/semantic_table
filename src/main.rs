mod expression;

use expression::Expression;

fn main() {
    // (a -> b) ^ (b -> a) ^ -a
    let omega = vec!["|-ab", "|a-b", "&ab", "-a"];
    let omega: Vec<Expression> = omega.iter().map(|expr| Expression::parse(expr)).collect();
    for expr in &omega {
        println!("{expr:?} -> {:?}", expr.rule());
    }
}
