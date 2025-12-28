macro_rules! deps {
    () => {
        CompilerInterface!();
        Error!();
    };
}

macro_rules! run {
    () => {
        deps!();
        pub (crate) fn run < F , T > (interface : & dyn CompilerInterface , f : F) -> Result < T , Error > where F : FnOnce () -> T , { if TLV . is_set () { Err (Error :: from ("rustc_public already running")) } else { let ptr : * const () = (& raw const interface) as _ ; TLV . set (& Cell :: new (ptr) , | | Ok (f ())) } }
    };
}

run!()