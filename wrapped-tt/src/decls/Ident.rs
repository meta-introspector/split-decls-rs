macro_rules! deps {
    () => {
        IdentIsRaw!();
    };
}

macro_rules! Ident {
    () => {
        deps!();
        # [doc = " Identifier or keyword."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Ident < S > { pub sym : Symbol , pub span : S , pub is_raw : IdentIsRaw , }
    };
}

Ident!()