macro_rules! deps {
    () => {
        ErasedFileAstId!();
        ErasedFileAstIdKind!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Debug for ErasedFileAstId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let kind = self . kind () ; macro_rules ! kind { ($ ($ kind : ident) ,* $ (,) ?) => { if false { match ErasedFileAstIdKind :: Root { $ (ErasedFileAstIdKind ::$ kind => { }) * } unreachable ! () } $ (else if kind == ErasedFileAstIdKind ::$ kind as u32 { stringify ! ($ kind) }) * else { "Unknown" } } ; } let kind = kind ! (Root , Enum , Struct , Union , ExternCrate , MacroDef , MacroRules , Module , Static , Trait , TraitAlias , Variant , Const , Fn , MacroCall , TypeAlias , ExternBlock , Use , Impl , BlockExpr , AsmExpr , Fixup ,) ; if f . alternate () { write ! (f , "{kind}[{:04X}, {}]" , self . hash_value () , self . index ()) } else { f . debug_struct ("ErasedFileAstId") . field ("kind" , & format_args ! ("{kind}")) . field ("index" , & self . index ()) . field ("hash" , & format_args ! ("{:04X}" , self . hash_value ())) . finish () } } }
    };
}

impl_3!();