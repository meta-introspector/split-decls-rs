macro_rules! deps {
    () => {
        MockCompilerHost!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < C > CompilerHost < C > for MockCompilerHost { fn run_compiler_callbacks (& self , _args : Vec < String > , _callbacks : & mut C) { println ! ("MockCompilerHost::run_compiler_callbacks called") ; } }
    };
}

impl_1!();