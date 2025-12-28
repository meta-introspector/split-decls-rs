macro_rules! arc {
    () => {
        # [cfg (target_has_atomic = "ptr")] mod arc ;
    };
}

arc!();