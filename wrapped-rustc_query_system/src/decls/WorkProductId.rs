macro_rules! WorkProductId {
    () => {
        # [doc = " A \"work product\" corresponds to a `.o` (or other) file that we"] # [doc = " save in between runs. These IDs do not have a `DefId` but rather"] # [doc = " some independent path or string that persists between runs without"] # [doc = " the need to be mapped or unmapped. (This ensures we can serialize"] # [doc = " them even in the absence of a tcx.)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable , Decodable)] pub struct WorkProductId { hash : Fingerprint , }
    };
}

WorkProductId!()