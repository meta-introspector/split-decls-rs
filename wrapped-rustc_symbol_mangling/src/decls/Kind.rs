macro_rules! Kind {
    () => {
        pub enum Kind { SymbolName , Demangling , DemanglingAlt , DefPath , }
    };
}

Kind!();