// Generated macro for macro_101 (macro)
macro_rules! Depcrate_builtinmacro_101 {
() => {
// Module: crate::builtin
// Provides: {"macro_101"}
// Dependencies: {}
declare_lint ! { # [doc = " The `hidden_glob_reexports` lint detects cases where glob re-export items are shadowed by"] # [doc = " private items."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(hidden_glob_reexports)]"] # [doc = ""] # [doc = " pub mod upstream {"] # [doc = "     mod inner { pub struct Foo {}; pub struct Bar {}; }"] # [doc = "     pub use self::inner::*;"] # [doc = "     struct Foo {} // private item shadows `inner::Foo`"] # [doc = " }"] # [doc = ""] # [doc = " // mod downstream {"] # [doc = " //     fn test() {"] # [doc = " //         let _ = crate::upstream::Foo; // inaccessible"] # [doc = " //     }"] # [doc = " // }"] # [doc = ""] # [doc = " pub fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This was previously accepted without any errors or warnings but it could silently break a"] # [doc = " crate's downstream user code. If the `struct Foo` was added, `dep::inner::Foo` would"] # [doc = " silently become inaccessible and trigger a \"`struct `Foo` is private`\" visibility error at"] # [doc = " the downstream use site."] pub HIDDEN_GLOB_REEXPORTS , Warn , "name introduced by a private item shadows a name introduced by a public glob re-export" , }
};
}
