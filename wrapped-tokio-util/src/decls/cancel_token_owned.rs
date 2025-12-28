macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! cancel_token_owned {
    () => {
        deps!();
        # [test] fn cancel_token_owned () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let th1 = thread :: spawn (move | | { block_on (async { token1 . cancelled_owned () . await ; }) ; }) ; let th2 = thread :: spawn (move | | { token . cancel () ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; }) ; }
    };
}

cancel_token_owned!()