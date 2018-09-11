extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate pest_deconstruct;

mod parser {
    #[derive(Parser)]
    #[grammar = "../examples/simple.pest"]
    pub struct MyParser;
    const _GRAMMAR: &str = include_str!("simple.pest");
}

mod ast {
    use super::parser::Rule;
    use pest::Span;

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::Term")]
    pub struct Term<'i> {
        span: Span<'i>,
        #[pest(parse)]
        value: u8,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::Expr")]
    pub struct Expr<'i> {
        span: Span<'i>,
        lhs: Term<'i>,
        op: Op<'i>,
        #[pest(parse, rule = "Rule::Term")]
        rhs: u8,
    }

    #[derive(Debug, FromPest)]
    #[pest(rule = "Rule::Op")]
    pub struct Op<'i> {
        span: Span<'i>,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use parser::{MyParser, Rule};
    use pest::Parser;
    use pest_deconstruct::FromPest;

    let parse = MyParser::parse(Rule::Expr, "9-7")?.next().unwrap();
    let expr = ast::Expr::from_pest(parse);
    println!("{:#?}", expr);
    Ok(())
}