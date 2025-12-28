macro_rules! deps {
    () => {
        Const!();
        Interner!();
        Ty!();
    };
}

macro_rules! TermKind {
    () => {
        deps!();
        # [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum TermKind < I : Interner > { Ty (I :: Ty) , Const (I :: Const) , }
    };
}

TermKind!()