macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! MacroCall {
    () => {
        deps!();
        # [doc = " A call to a namespaced macro `macros::my_macro()`"] # [derive (Clone , Debug , PartialEq)] pub struct MacroCall { # [doc = " The namespace we're looking for that macro in"] pub namespace : String , # [doc = " The macro name"] pub name : String , # [doc = " The args for that macro: name -> value"] pub args : HashMap < String , Expr > , }
    };
}

MacroCall!();