macro_rules! pause {
    () => {
        # [cfg (not (any (windows , target_os = "redox" , target_os = "wasi")))] mod pause ;
    };
}

pause!();