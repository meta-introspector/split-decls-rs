macro_rules! impl_85 {
    () => {
        impl gz_header { # [allow (clippy :: if_same_then_else)] pub const OS_CODE : u8 = { if cfg ! (windows) { 10 } else if cfg ! (target_os = "macos") { 19 } else if cfg ! (unix) { 3 } else { 3 } } ; pub (crate) fn flags (& self) -> u8 { (if self . text != 0 { 1 } else { 0 }) + (if self . hcrc != 0 { 2 } else { 0 }) + (if self . extra . is_null () { 0 } else { 4 }) + (if self . name . is_null () { 0 } else { 8 }) + (if self . comment . is_null () { 0 } else { 16 }) } }
    };
}

impl_85!()