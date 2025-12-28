macro_rules! deps {
    () => {
        BoundVar!();
        BoundRegionKind!();
    };
}

macro_rules! BoundRegion {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct BoundRegion { pub var : BoundVar , pub kind : BoundRegionKind , }
    };
}

BoundRegion!();