macro_rules! check_keyword_matches {
    () => {
        macro_rules ! check_keyword_matches { (enum enum) => { } ; (pub pub) => { } ; (struct struct) => { } ; }
    };
}

check_keyword_matches!()