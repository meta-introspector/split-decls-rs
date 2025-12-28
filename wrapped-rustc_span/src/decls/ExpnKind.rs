macro_rules! deps {
    () => {
        AstPass!();
        Symbol!();
        DesugaringKind!();
        MacroKind!();
    };
}

macro_rules! ExpnKind {
    () => {
        deps!();
        # [doc = " Expansion kind."] # [derive (Clone , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum ExpnKind { # [doc = " No expansion, aka root expansion. Only `ExpnId::root()` has this kind."] Root , # [doc = " Expansion produced by a macro."] Macro (MacroKind , Symbol) , # [doc = " Transform done by the compiler on the AST."] AstPass (AstPass) , # [doc = " Desugaring done by the compiler during AST lowering."] Desugaring (DesugaringKind) , }
    };
}

ExpnKind!()