macro_rules! deps {
    () => {
        RegionKind!();
    };
}

macro_rules! Region {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Region { pub kind : RegionKind , }
    };
}

Region!()