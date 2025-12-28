macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! cancel_with_child {
    () => {
        deps!();
        # [test] fn cancel_with_child () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let token2 = token . clone () ; let child_token = token . child_token () ; let th1 = thread :: spawn (move | | { block_on (async { token1 . cancelled () . await ; }) ; }) ; let th2 = thread :: spawn (move | | { token2 . cancel () ; }) ; let th3 = thread :: spawn (move | | { block_on (async { child_token . cancelled () . await ; }) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
    };
}

cancel_with_child!();