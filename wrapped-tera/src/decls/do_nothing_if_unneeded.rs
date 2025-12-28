macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! do_nothing_if_unneeded {
    () => {
        deps!();
        # [test] fn do_nothing_if_unneeded () { let ast = vec ! [Node :: Text ("hey " . to_string ())] ; assert_eq ! (remove_whitespace (ast . clone () , None) , ast) ; }
    };
}

do_nothing_if_unneeded!()