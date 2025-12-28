macro_rules! macro_48 {
    () => {
        # [cfg (target_pointer_width = "16")] compile_error ! ("text-size assumes usize >= u32 and does not work on 16-bit targets") ;
    };
}

macro_48!();