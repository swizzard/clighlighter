fn main() {
    let s = include_str!("index.ts");
    println!("{}", clighlighter::highlight(s));
}
