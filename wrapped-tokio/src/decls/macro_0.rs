macro_rules! macro_0 {
    () => {
        # [cfg (not (any (target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ! { "Tokio requires the platform pointer width to be at least 32 bits" }
    };
}

macro_0!()