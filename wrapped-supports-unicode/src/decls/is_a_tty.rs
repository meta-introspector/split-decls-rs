macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! is_a_tty {
    () => {
        deps!();
        fn is_a_tty (stream : Stream) -> bool { use std :: io :: IsTerminal ; match stream { Stream :: Stdout => std :: io :: stdout () . is_terminal () , Stream :: Stderr => std :: io :: stderr () . is_terminal () , } }
    };
}

is_a_tty!();