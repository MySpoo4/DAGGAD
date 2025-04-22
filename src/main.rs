use twdawg::TWDawgBuilder;

// Example
fn main() {
    let words = ["sad", "dog", "ocaml", "node", "dagger", "nod"];
    let tw_dawg = TWDawgBuilder::build(words);

    let mut all_words = tw_dawg.get_all_words();
    all_words.sort();

    println!("{:#?}", tw_dawg);
    println!("all_words: {:#?}", all_words);
}
