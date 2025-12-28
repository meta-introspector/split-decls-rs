macro_rules! deps {
    () => {
        Span!();
        Symbol!();
    };
}

macro_rules! Ident {
    () => {
        deps!();
        # [derive (Copy , Clone , Eq , HashStable_Generic , Encodable , Decodable)] pub struct Ident { pub name : Symbol , pub span : Span , }
    };
}

Ident!();