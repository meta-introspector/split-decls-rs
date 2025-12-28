macro_rules! pivot_root {
    () => {
        # [cfg (target_os = "linux")] mod pivot_root ;
    };
}

pivot_root!()