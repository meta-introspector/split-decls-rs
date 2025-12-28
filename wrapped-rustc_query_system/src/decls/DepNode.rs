macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! DepNode {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct DepNode { pub kind : DepKind , pub hash : PackedFingerprint , }
    };
}

DepNode!()