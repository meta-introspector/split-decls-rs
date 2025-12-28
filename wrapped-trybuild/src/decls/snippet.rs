macro_rules! snippet {
    () => {
        fn snippet (color : Color , content : & str) { snippet_diff (color , content , None) ; }
    };
}

snippet!()