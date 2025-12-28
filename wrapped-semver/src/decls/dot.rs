macro_rules! deps {
    () => {
        Position!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! dot {
    () => {
        deps!();
        fn dot (input : & str , pos : Position) -> Result < & str , Error > { if let Some (rest) = input . strip_prefix ('.') { Ok (rest) } else if let Some (unexpected) = input . chars () . next () { Err (Error :: new (ErrorKind :: UnexpectedCharAfter (pos , unexpected))) } else { Err (Error :: new (ErrorKind :: UnexpectedEnd (pos))) } }
    };
}

dot!();