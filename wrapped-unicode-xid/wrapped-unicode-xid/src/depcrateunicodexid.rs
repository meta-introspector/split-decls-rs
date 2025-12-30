// Generated macro for UnicodeXID (trait)
macro_rules! DepcrateUnicodeXID {
() => {
// Module: crate
// Provides: {"UnicodeXID"}
// Dependencies: {}
# [doc = " Methods for determining if a character is a valid identifier character."] pub trait UnicodeXID { # [doc = " Returns whether the specified character satisfies the 'XID_Start'"] # [doc = " Unicode property."] # [doc = ""] # [doc = " 'XID_Start' is a Unicode Derived Property specified in"] # [doc = " [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),"] # [doc = " mostly similar to ID_Start but modified for closure under NFKx."] fn is_xid_start (self) -> bool ; # [doc = " Returns whether the specified `char` satisfies the 'XID_Continue'"] # [doc = " Unicode property."] # [doc = ""] # [doc = " 'XID_Continue' is a Unicode Derived Property specified in"] # [doc = " [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),"] # [doc = " mostly similar to 'ID_Continue' but modified for closure under NFKx."] fn is_xid_continue (self) -> bool ; }
};
}
