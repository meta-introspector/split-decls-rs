macro_rules! deps {
    () => {
        AstPass!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl AstPass { pub fn descr (self) -> & 'static str { match self { AstPass :: StdImports => "standard library imports" , AstPass :: TestHarness => "test harness" , AstPass :: ProcMacroHarness => "proc macro harness" , } } }
    };
}

impl_73!()