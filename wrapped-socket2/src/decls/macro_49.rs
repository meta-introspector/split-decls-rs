macro_rules! macro_49 {
    () => {
        # [cfg (not (any (windows , unix)))] compile_error ! ("Socket2 doesn't support the compile target") ;
    };
}

macro_49!();