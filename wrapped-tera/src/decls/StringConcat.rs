macro_rules! deps {
    () => {
        ExprVal!();
    };
}

macro_rules! StringConcat {
    () => {
        deps!();
        # [doc = " Can only be a combination of string + ident or ident + ident"] # [derive (Clone , Debug , PartialEq)] pub struct StringConcat { # [doc = " All the values we're concatening into a string"] pub values : Vec < ExprVal > , }
    };
}

StringConcat!()