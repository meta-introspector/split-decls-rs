macro_rules! deps {
    () => {
        Error!();
        Suggestion!();
        Solution!();
        Data!();
        CodeFix!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl CodeFix { # [doc = " Creates a `CodeFix` with the source of a file to modify."] pub fn new (s : & str) -> CodeFix { CodeFix { data : replace :: Data :: new (s . as_bytes ()) , modified : false , } } # [doc = " Applies a suggestion to the code."] pub fn apply (& mut self , suggestion : & Suggestion) -> Result < () , Error > { for solution in & suggestion . solutions { for r in & solution . replacements { self . data . replace_range (r . snippet . range . clone () , r . replacement . as_bytes ()) . inspect_err (| _ | self . data . restore ()) ? ; } } self . data . commit () ; self . modified = true ; Ok (()) } # [doc = " Applies an individual solution from a [`Suggestion`]."] pub fn apply_solution (& mut self , solution : & Solution) -> Result < () , Error > { for r in & solution . replacements { self . data . replace_range (r . snippet . range . clone () , r . replacement . as_bytes ()) . inspect_err (| _ | self . data . restore ()) ? ; } self . data . commit () ; self . modified = true ; Ok (()) } # [doc = " Gets the result of the \"fixed\" code."] pub fn finish (& self) -> Result < String , Error > { Ok (String :: from_utf8 (self . data . to_vec ()) ?) } # [doc = " Returns whether or not the data has been modified."] pub fn modified (& self) -> bool { self . modified } }
    };
}

impl_31!();