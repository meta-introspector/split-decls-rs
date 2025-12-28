macro_rules! deps {
    () => {
        Region!();
        Interner!();
        Const!();
        Ty!();
    };
}

macro_rules! GenericArgKind {
    () => {
        deps!();
        # [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum GenericArgKind < I : Interner > { Lifetime (I :: Region) , Type (I :: Ty) , Const (I :: Const) , }
    };
}

GenericArgKind!()