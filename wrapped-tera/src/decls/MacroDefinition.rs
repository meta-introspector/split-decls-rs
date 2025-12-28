macro_rules! deps {
    () => {
        Expr!();
        Node!();
    };
}

macro_rules! MacroDefinition {
    () => {
        deps!();
        # [doc = " A Macro definition"] # [derive (Clone , Debug , PartialEq)] pub struct MacroDefinition { # [doc = " The macro name"] pub name : String , # [doc = " The args for that macro: name -> optional default value"] pub args : HashMap < String , Option < Expr > > , # [doc = " The macro content"] pub body : Vec < Node > , }
    };
}

MacroDefinition!();