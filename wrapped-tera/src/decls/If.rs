macro_rules! deps {
    () => {
        WS!();
        Expr!();
        Node!();
    };
}

macro_rules! If {
    () => {
        deps!();
        # [doc = " An if/elif/else condition with their respective body"] # [derive (Clone , Debug , PartialEq)] pub struct If { # [doc = " First item if the if, all the ones after are elif"] pub conditions : Vec < (WS , Expr , Vec < Node >) > , # [doc = " The optional `else` block"] pub otherwise : Option < (WS , Vec < Node >) > , }
    };
}

If!();