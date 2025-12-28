macro_rules! deps {
    () => {
        SizedTraitKind!();
        EarlyBinder!();
        AdtDestructorKind!();
        Ty!();
        Interner!();
    };
}

macro_rules! AdtDef {
    () => {
        deps!();
        pub trait AdtDef < I : Interner > : Copy + Debug + Hash + Eq { fn def_id (self) -> I :: AdtId ; fn is_struct (self) -> bool ; # [doc = " Returns the type of the struct tail."] # [doc = ""] # [doc = " Expects the `AdtDef` to be a struct. If it is not, then this will panic."] fn struct_tail_ty (self , interner : I) -> Option < ty :: EarlyBinder < I , I :: Ty > > ; fn is_phantom_data (self) -> bool ; fn is_manually_drop (self) -> bool ; fn all_field_tys (self , interner : I) -> ty :: EarlyBinder < I , impl IntoIterator < Item = I :: Ty > > ; fn sizedness_constraint (self , interner : I , sizedness : SizedTraitKind ,) -> Option < ty :: EarlyBinder < I , I :: Ty > > ; fn is_fundamental (self) -> bool ; fn destructor (self , interner : I) -> Option < AdtDestructorKind > ; }
    };
}

AdtDef!()