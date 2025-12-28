macro_rules! LitStrOrIdent {
    () => {
        # [derive (Debug , Clone)] pub (super) enum LitStrOrIdent { LitStr (LitStr) , Ident (Ident) , }
    };
}

LitStrOrIdent!();