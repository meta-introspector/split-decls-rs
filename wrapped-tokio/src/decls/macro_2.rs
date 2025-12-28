macro_rules! macro_2 {
    () => {
        # [cfg (all (not (tokio_unstable) , feature = "io-uring"))] compile_error ! ("The `io-uring` feature requires `--cfg tokio_unstable`.") ;
    };
}

macro_2!();