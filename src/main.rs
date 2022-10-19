mod expression;
mod semantic_table;

use expression::Expression;
use semantic_table::SemanticTable;

fn main() {
    // (a -> b) ^ (b -> a) ^ -a
    let expressions = vec!["|-ab", "|a-b", "&ab"];
    let omega = SemanticTable::parse_set(&expressions);
    for expr in omega.expressions() {
        println!("{}", expr.to_string());
    }
    omega.models(Expression::parse("-a").0);
}
