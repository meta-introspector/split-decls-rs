macro_rules! sort_by_words {
    () => {
        fn sort_by_words (name : & str) -> Vec < & str > { let mut split_words : Vec < & str > = name . split ('_') . collect () ; split_words . sort_unstable () ; split_words }
    };
}

sort_by_words!();