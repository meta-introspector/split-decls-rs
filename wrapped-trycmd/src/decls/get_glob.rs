macro_rules! get_glob {
    () => {
        fn get_glob (path : & std :: path :: Path) -> Option < & str > { if let Some (utf8) = path . to_str () { if utf8 . contains ('*') { return Some (utf8) ; } } None }
    };
}

get_glob!();