pub mod io;
pub mod token;
pub mod lexer;


pub fn compile() {
    let source      = io::Source::new("examples/main.lace");
    let mut lexer   = lexer::Lexer::new(source);
    let mut current = lexer.next_token();

    while current.kind != token::Kind::EOF {
        println!("{current:?}");
        current     = lexer.next_token();
    }
}
