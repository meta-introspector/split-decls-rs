macro_rules! RUSTVERSION {
    () => {
        # [cfg (host_os = "windows")] const RUSTVERSION : Version = include ! (concat ! (env ! ("OUT_DIR") , "\\version.expr")) ;
    };
}

RUSTVERSION!()