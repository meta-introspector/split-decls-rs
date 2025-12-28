macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! FunctionCall {
    () => {
        deps!();
        # [doc = " A function call, can be a filter or a global function"] # [derive (Clone , Debug , PartialEq)] pub struct FunctionCall { # [doc = " The name of the function"] pub name : String , # [doc = " The args of the function: key -> value"] pub args : HashMap < String , Expr > , }
    };
}

FunctionCall!();