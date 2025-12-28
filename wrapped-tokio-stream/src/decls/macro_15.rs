macro_rules! macro_15 {
    () => {
        cfg_fs ! { # [cfg (not (loom))] mod read_dir ; # [cfg (not (loom))] pub use read_dir :: ReadDirStream ; }
    };
}

macro_15!();