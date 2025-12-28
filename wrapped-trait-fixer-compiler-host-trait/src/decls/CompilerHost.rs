macro_rules! CompilerHost {
    () => {
        pub trait CompilerHost < C > { fn run_compiler_callbacks (& self , args : Vec < String > , callbacks : & mut C) ; }
    };
}

CompilerHost!();