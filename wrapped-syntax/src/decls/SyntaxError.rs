macro_rules! SyntaxError {
    () => {
        # [doc = " Represents the result of unsuccessful tokenization, parsing"] # [doc = " or tree validation."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct SyntaxError (String , TextRange) ;
    };
}

SyntaxError!()