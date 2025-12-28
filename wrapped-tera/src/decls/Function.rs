macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! Function {
    () => {
        deps!();
        # [doc = " The global function type definition"] pub trait Function : Sync + Send { # [doc = " The global function type definition"] fn call (& self , args : & HashMap < String , Value >) -> Result < Value > ; # [doc = " Whether the current function's output should be treated as safe, defaults to `false`"] fn is_safe (& self) -> bool { false } }
    };
}

Function!();