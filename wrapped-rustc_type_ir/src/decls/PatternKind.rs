macro_rules! deps {
    () => {
        Interner!();
        Const!();
    };
}

macro_rules! PatternKind {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum PatternKind < I : Interner > { Range { start : I :: Const , end : I :: Const } , Or (I :: PatList) , }
    };
}

PatternKind!()