// Generated macro for use_513 (pub_use)
macro_rules! Depcrateuse_513 {
() => {
// Module: crate
// Provides: {"use_513"}
// Dependencies: {}
# [doc = " Derives optimized [`PartialEq`] and [`Eq`] implementations."] # [doc = ""] # [doc = " This derive can be applied to structs and enums implementing both"] # [doc = " [`Immutable`] and [`IntoBytes`]; e.g.:"] # [doc = ""] # [doc = " ```"] # [doc = " # use zerocopy_derive::{ByteEq, Immutable, IntoBytes};"] # [doc = " #[derive(ByteEq, Immutable, IntoBytes)]"] # [doc = " #[repr(C)]"] # [doc = " struct MyStruct {"] # [doc = " # /*"] # [doc = "     ..."] # [doc = " # */"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(ByteEq, Immutable, IntoBytes)]"] # [doc = " #[repr(u8)]"] # [doc = " enum MyEnum {"] # [doc = " #   Variant,"] # [doc = " # /*"] # [doc = "     ..."] # [doc = " # */"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The standard library's [`derive(Eq, PartialEq)`][derive@PartialEq] computes"] # [doc = " equality by individually comparing each field. Instead, the implementation"] # [doc = " of [`PartialEq::eq`] emitted by `derive(ByteHash)` converts the entirety of"] # [doc = " `self` and `other` to byte slices and compares those slices for equality."] # [doc = " This may have performance advantages."] # [cfg (any (feature = "derive" , test))] # [cfg_attr (doc_cfg , doc (cfg (feature = "derive")))] pub use zerocopy_derive :: ByteEq ;
};
}
