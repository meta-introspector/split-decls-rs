macro_rules! AttributeReader {
    () => {
        pub trait AttributeReader < 'tcx > { type DefId : 'tcx ; type Symbol ; fn has_derive_attr (& 'tcx self , def_id : Self :: DefId , trait_name : & str) -> bool ; fn sym_derive () -> Self :: Symbol ; fn sym_intern (s : & str) -> Self :: Symbol ; }
    };
}

AttributeReader!();