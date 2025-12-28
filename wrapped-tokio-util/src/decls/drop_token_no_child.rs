macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! drop_token_no_child {
    () => {
        deps!();
        # [test] fn drop_token_no_child () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let token2 = token . clone () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { drop (token2) ; }) ; let th3 = thread :: spawn (move | | { drop (token) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
    };
}

drop_token_no_child!();