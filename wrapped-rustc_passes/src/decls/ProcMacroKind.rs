macro_rules! ProcMacroKind {
    () => {
        # [derive (Copy , Clone)] pub (crate) enum ProcMacroKind { FunctionLike , Derive , Attribute , }
    };
}

ProcMacroKind!();