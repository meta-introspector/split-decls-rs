macro_rules! check_file_invariants {
    () => {
        fn check_file_invariants (file : & SourceFile) { let root = file . syntax () ; validation :: validate_block_structure (root) ; }
    };
}

check_file_invariants!()