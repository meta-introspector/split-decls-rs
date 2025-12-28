macro_rules! OpaqueId {
    () => {
        # [doc = " A globally unique id to distinguish `Opaque` patterns."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct OpaqueId (u32) ;
    };
}

OpaqueId!()