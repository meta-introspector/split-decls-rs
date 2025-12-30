// Generated macro for msg (function)
macro_rules! Depcrate_helpersmsg {
() => {
// Module: crate::helpers
// Provides: {"msg"}
// Dependencies: {}
fn msg (expected : & [String] , found : Option < Arg >) -> String { if expected . is_empty () { return "unexpected token." . into () ; } let mut m = String :: new () ; m . push_str ("expected ") ; for i in 0 .. expected . len () { if i != 0 { let sep = if i == expected . len () - 1 { " or " } else { ", " } ; m . push_str (sep) ; } m . push_str (& expected [i]) ; } if let Some (arg) = found { m . push_str (", found ") ; m . push_str (& match arg . kind { ArgKind :: Flag => format ! ("`{}`" , arg . ident) , ArgKind :: NameValue => format ! ("`{} = ...`" , arg . ident) , ArgKind :: NameArgs => format ! ("`{}`(...)" , arg . ident) , }) ; } m }
};
}
