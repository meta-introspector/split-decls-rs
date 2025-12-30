// Generated macro for PrefixComponent (struct)
macro_rules! Depcrate_pathPrefixComponent {
() => {
// Module: crate::path
// Provides: {"PrefixComponent"}
// Dependencies: {}
# [doc = " A structure wrapping a Windows path prefix as well as its unparsed string"] # [doc = " representation."] # [doc = ""] # [doc = " In addition to the parsed [`Prefix`] information returned by [`kind`],"] # [doc = " `PrefixComponent` also holds the raw and unparsed [`OsStr`] slice,"] # [doc = " returned by [`as_os_str`]."] # [doc = ""] # [doc = " Instances of this `struct` can be obtained by matching against the"] # [doc = " [`Prefix` variant] on [`Component`]."] # [doc = ""] # [doc = " Does not occur on Unix."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(windows) {"] # [doc = " use std::path::{Component, Path, Prefix};"] # [doc = " use std::ffi::OsStr;"] # [doc = ""] # [doc = " let path = Path::new(r\"c:\\you\\later\\\");"] # [doc = " match path.components().next().unwrap() {"] # [doc = "     Component::Prefix(prefix_component) => {"] # [doc = "         assert_eq!(Prefix::Disk(b'C'), prefix_component.kind());"] # [doc = "         assert_eq!(OsStr::new(\"c:\"), prefix_component.as_os_str());"] # [doc = "     }"] # [doc = "     _ => unreachable!(),"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [`as_os_str`]: PrefixComponent::as_os_str"] # [doc = " [`kind`]: PrefixComponent::kind"] # [doc = " [`Prefix` variant]: Component::Prefix"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Copy , Clone , Eq , Debug)] pub struct PrefixComponent < 'a > { # [doc = " The prefix as an unparsed `OsStr` slice."] raw : & 'a OsStr , # [doc = " The parsed prefix data."] parsed : Prefix < 'a > , }
};
}
