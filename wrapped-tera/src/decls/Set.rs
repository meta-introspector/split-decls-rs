macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! Set {
    () => {
        deps!();
        # [doc = " Set a variable in the context `{% set val = \"hey\" %}`"] # [derive (Clone , Debug , PartialEq)] pub struct Set { # [doc = " The name for that value in the context"] pub key : String , # [doc = " The value to assign"] pub value : Expr , # [doc = " Whether we want to set the variable globally or locally"] # [doc = " global_set is only useful in loops"] pub global : bool , }
    };
}

Set!()