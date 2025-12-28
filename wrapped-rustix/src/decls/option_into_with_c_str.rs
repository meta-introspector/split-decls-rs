macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! option_into_with_c_str {
    () => {
        deps!();
        # [doc = " Runs a closure on `arg` where `A` is mapped to a `&CStr`"] pub fn option_into_with_c_str < T , F , A > (arg : Option < A > , f : F) -> io :: Result < T > where A : Arg + Sized , F : FnOnce (Option < & CStr >) -> io :: Result < T > , { if let Some (arg) = arg { arg . into_with_c_str (| p | f (Some (p))) } else { f (None) } }
    };
}

option_into_with_c_str!();