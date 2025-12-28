macro_rules! AstPass {
    () => {
        # [doc = " The kind of AST transform."] # [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum AstPass { StdImports , TestHarness , ProcMacroHarness , }
    };
}

AstPass!()