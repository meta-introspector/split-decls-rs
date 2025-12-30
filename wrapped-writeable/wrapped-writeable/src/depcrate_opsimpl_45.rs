// Generated macro for impl_45 (impl)
macro_rules! Depcrate_opsimpl_45 {
() => {
// Module: crate::ops
// Provides: {"impl_45"}
// Dependencies: {}
impl core :: ops :: BitOr < LengthHint > for LengthHint { type Output = Self ; # [doc = " Returns a new hint that is correct wherever `self` is correct, and wherever"] # [doc = " `other` is correct."] # [doc = ""] # [doc = " Example:"] # [doc = " ```"] # [doc = " # use writeable::{LengthHint, Writeable};"] # [doc = " # use core::fmt;"] # [doc = " # fn coin_flip() -> bool { true }"] # [doc = ""] # [doc = " struct NonDeterministicWriteable(String, String);"] # [doc = ""] # [doc = " impl Writeable for NonDeterministicWriteable {"] # [doc = "     fn write_to<W: fmt::Write + ?Sized>("] # [doc = "         &self,"] # [doc = "         sink: &mut W,"] # [doc = "     ) -> fmt::Result {"] # [doc = "         sink.write_str(if coin_flip() { &self.0 } else { &self.1 })"] # [doc = "     }"] # [doc = ""] # [doc = "     fn writeable_length_hint(&self) -> LengthHint {"] # [doc = "         LengthHint::exact(self.0.len()) | LengthHint::exact(self.1.len())"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " writeable::impl_display_with_writeable!(NonDeterministicWriteable);"] # [doc = " ```"] fn bitor (self , other : LengthHint) -> Self { LengthHint (Ord :: min (self . 0 , other . 0) , match (self . 1 , other . 1) { (Some (c) , Some (d)) => Some (Ord :: max (c , d)) , _ => None , } ,) } }
};
}
