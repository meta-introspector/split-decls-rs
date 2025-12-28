macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! drop_token_with_children {
    () => {
        deps!();
        # [ignore] # [test] fn drop_token_with_children () { loom :: model (| | { let token1 = CancellationToken :: new () ; let child_token1 = token1 . child_token () ; let child_token2 = token1 . child_token () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { drop (child_token1) ; }) ; let th3 = thread :: spawn (move | | { drop (child_token2) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
    };
}

drop_token_with_children!();