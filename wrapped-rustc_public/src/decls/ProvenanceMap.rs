macro_rules! deps {
    () => {
        Size!();
        Prov!();
    };
}

macro_rules! ProvenanceMap {
    () => {
        deps!();
        # [doc = " Stores the provenance information of pointers stored in memory."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ProvenanceMap { # [doc = " Provenance in this map applies from the given offset for an entire pointer-size worth of"] # [doc = " bytes. Two entries in this map are always at least a pointer size apart."] pub ptrs : Vec < (Size , Prov) > , }
    };
}

ProvenanceMap!()