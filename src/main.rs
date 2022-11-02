pub mod lace;


fn main() {
    let source = lace::io::Source::new("src/main.rs");
    println!("{source:?}");
}
