macro_rules! Quantifier {
    () => {
        enum Quantifier { ThereExists , ForAll , }
    };
}

Quantifier!()