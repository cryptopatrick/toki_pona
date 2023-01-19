mod vocabulary;
use vocabulary::{Vocabulary};

fn main() {
    let vocab = Vocabulary::new();
    vocab.print_vocabulary();
}