macro_rules! macro_3 {
    () => {
        # [cfg (all (not (tokio_unstable) , feature = "taskdump"))] compile_error ! ("The `taskdump` feature requires `--cfg tokio_unstable`.") ;
    };
}

macro_3!()