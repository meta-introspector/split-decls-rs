macro_rules! priority {
    () => {
        # [cfg (not (any (target_os = "fuchsia" , target_os = "vita" , target_os = "wasi")))] mod priority ;
    };
}

priority!();