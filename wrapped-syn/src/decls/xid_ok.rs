macro_rules! xid_ok {
    () => {
        pub (crate) fn xid_ok (symbol : & str) -> bool { let mut chars = symbol . chars () ; let first = chars . next () . unwrap () ; if ! (first == '_' || unicode_ident :: is_xid_start (first)) { return false ; } for ch in chars { if ! unicode_ident :: is_xid_continue (ch) { return false ; } } true }
    };
}

xid_ok!();