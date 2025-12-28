macro_rules! Mismatch {
    () => {
        struct Mismatch { slug_name : String , crate_name : String , slug_prefix : String , }
    };
}

Mismatch!();