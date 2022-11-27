use while_lang::{Lexer, Parser, Interpreter};

fn main() {
    let input = include_str!("../sample.while");
    let tokens = Lexer::new(input).lex();
    let tree = Parser::new(tokens).parse();
    let result = Interpreter::new(&[3, 6]).run(tree);
    dbg!(result);
}
