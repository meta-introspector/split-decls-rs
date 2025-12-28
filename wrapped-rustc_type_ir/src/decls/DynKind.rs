macro_rules! DynKind {
    () => {
        # [doc = " Specifies how a trait object is represented."] # [doc = ""] # [doc = " This used to have a variant `DynStar`, but that variant has been removed,"] # [doc = " and it's likely this whole enum will be removed soon."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum DynKind { # [doc = " An unsized `dyn Trait` object"] Dyn , }
    };
}

DynKind!()