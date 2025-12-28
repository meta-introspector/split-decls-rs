macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! validate_block_structure {
    () => {
        deps!();
        pub (crate) fn validate_block_structure (root : & SyntaxNode) { let mut stack = Vec :: new () ; for node in root . descendants_with_tokens () { match node . kind () { T ! ['{'] => stack . push (node) , T ! ['}'] => { if let Some (pair) = stack . pop () { assert_eq ! (node . parent () , pair . parent () , "\nunpaired curlies:\n{}\n{:#?}\n" , root . text () , root ,) ; assert ! (node . next_sibling_or_token () . is_none () && pair . prev_sibling_or_token () . is_none () , "\nfloating curlies at {:?}\nfile:\n{}\nerror:\n{}\n" , node , root . text () , node ,) ; } } _ => () , } } }
    };
}

validate_block_structure!();