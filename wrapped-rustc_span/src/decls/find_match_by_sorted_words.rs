macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! find_match_by_sorted_words {
    () => {
        deps!();
        fn find_match_by_sorted_words (iter_names : & [Symbol] , lookup : & str) -> Option < Symbol > { let lookup_sorted_by_words = sort_by_words (lookup) ; iter_names . iter () . fold (None , | result , candidate | { if sort_by_words (candidate . as_str ()) == lookup_sorted_by_words { Some (* candidate) } else { result } }) }
    };
}

find_match_by_sorted_words!()