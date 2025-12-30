// Generated macro for impl_fmt_from (macro)
macro_rules! Depcrate_fmtimpl_fmt_from {
() => {
// Module: crate::fmt
// Provides: {"impl_fmt_from"}
// Dependencies: {}
macro_rules ! impl_fmt_from { ($ T : ident <>) => { impl From < Uuid > for $ T { # [inline] fn from (f : Uuid) -> Self { $ T (f) } } impl From <$ T > for Uuid { # [inline] fn from (f : $ T) -> Self { f . into_uuid () } } impl AsRef < Uuid > for $ T { # [inline] fn as_ref (& self) -> & Uuid { & self . 0 } } impl Borrow < Uuid > for $ T { # [inline] fn borrow (& self) -> & Uuid { & self . 0 } } } ; ($ T : ident <$ a : lifetime >) => { impl <$ a > From <&$ a Uuid > for $ T <$ a > { # [inline] fn from (f : &$ a Uuid) -> Self { $ T :: from_uuid_ref (f) } } impl <$ a > From <$ T <$ a >> for &$ a Uuid { # [inline] fn from (f : $ T <$ a >) -> &$ a Uuid { f . 0 } } impl <$ a > AsRef < Uuid > for $ T <$ a > { # [inline] fn as_ref (& self) -> & Uuid { self . 0 } } impl <$ a > Borrow < Uuid > for $ T <$ a > { # [inline] fn borrow (& self) -> & Uuid { self . 0 } } } ; }
};
}
