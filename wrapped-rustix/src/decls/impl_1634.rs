macro_rules! deps {
    () => {
        DecInt!();
    };
}

macro_rules! impl_1634 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg (any (not (target_os = "wasi") , not (target_env = "p2") , wasip2))] impl AsRef < Path > for DecInt { # [inline] fn as_ref (& self) -> & Path { let as_os_str : & OsStr = OsStrExt :: from_bytes (self . as_bytes ()) ; Path :: new (as_os_str) } }
    };
}

impl_1634!()