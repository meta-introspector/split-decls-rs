macro_rules! check_parser {
    () => {
        pub fn check_parser (text : & str) { let file = SourceFile :: parse (text , Edition :: CURRENT) ; check_file_invariants (& file . tree ()) ; }
    };
}

check_parser!()