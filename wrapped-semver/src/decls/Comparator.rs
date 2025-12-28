macro_rules! deps {
    () => {
        Op!();
        Prerelease!();
        VersionReq!();
    };
}

macro_rules! Comparator {
    () => {
        deps!();
        # [doc = " A pair of comparison operator and partial version, such as `>=1.2`. Forms"] # [doc = " one piece of a VersionReq."] # [derive (Clone , Eq , PartialEq , Hash , Debug)] pub struct Comparator { pub op : Op , pub major : u64 , pub minor : Option < u64 > , # [doc = " Patch is only allowed if minor is Some."] pub patch : Option < u64 > , # [doc = " Non-empty pre-release is only allowed if patch is Some."] pub pre : Prerelease , }
    };
}

Comparator!();