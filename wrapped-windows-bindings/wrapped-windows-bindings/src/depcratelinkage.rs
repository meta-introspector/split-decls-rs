// Generated macro for LINKAGE (const)
macro_rules! DepcrateLINKAGE {
() => {
// Module: crate
// Provides: {"LINKAGE"}
// Dependencies: {}
const LINKAGE : & [u8] = br#"
// jni-rs specific additions vendored from https://github.com/microsoft/windows-rs/blob/master/crates/libs/link/src/lib.rs
mod windows_targets {
    #[cfg(all(windows, target_arch = "x86"))]
    macro_rules! win_link {
        ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
            #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
            extern $abi {
                $(#[link_name=$link_name])?
                pub fn $($function)*;
            }
        )
    }
    #[cfg(all(windows, not(target_arch = "x86")))]
    macro_rules! win_link {
        ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
            #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
            extern $abi {
                $(#[link_name=$link_name])?
                pub fn $($function)*;
            }
        )
    }
    pub(super) use win_link as link;
}
"# ;
};
}
