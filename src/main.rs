use daggad::DaggadBuilder;

// Example
fn main() {
    let words = ["sad", "dog", "ocaml", "node", "dagger", "nod"];
    let daggad = DaggadBuilder::build(words);

    let mut all_words = daggad.get_all_words();
    all_words.sort();

    println!("{:#?}", daggad);
    println!("all_words: {:#?}", all_words);
}
