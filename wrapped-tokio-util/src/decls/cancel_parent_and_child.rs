macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! cancel_parent_and_child {
    () => {
        deps!();
        # [ignore] # [test] fn cancel_parent_and_child () { loom :: model (| | { let token1 = CancellationToken :: new () ; let token2 = token1 . clone () ; let child_token = token1 . child_token () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { token2 . cancel () ; }) ; let th3 = thread :: spawn (move | | { child_token . cancel () ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
    };
}

cancel_parent_and_child!()