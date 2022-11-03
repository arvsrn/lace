pub mod lace;


fn main() {
    let _string = lace::io::Source::new(&String::from("src/main.rs"));
    let _str = lace::io::Source::new("src/main.rs");
}
