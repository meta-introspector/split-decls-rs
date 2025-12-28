macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! RUSTVERSION {
    () => {
        deps!();
        # [cfg (host_os = "windows")] const RUSTVERSION : Version = include ! (concat ! (env ! ("OUT_DIR") , "\\version.expr")) ;
    };
}

RUSTVERSION!();