// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a > HyperlinkSpec < 'a > { # [doc = " Creates a new hyperlink specification."] pub fn open (uri : & 'a [u8]) -> HyperlinkSpec < 'a > { HyperlinkSpec { uri : Some (uri) } } # [doc = " Creates a hyperlink specification representing no hyperlink."] pub fn close () -> HyperlinkSpec < 'a > { HyperlinkSpec { uri : None } } # [doc = " Returns the URI of the hyperlink if one is attached to this spec."] pub fn uri (& self) -> Option < & 'a [u8] > { self . uri } }
};
}
