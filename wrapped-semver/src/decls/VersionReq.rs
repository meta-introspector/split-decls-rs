macro_rules! deps {
    () => {
        Op!();
        Comparator!();
    };
}

macro_rules! VersionReq {
    () => {
        deps!();
        # [doc = " **SemVer version requirement** describing the intersection of some version"] # [doc = " comparators, such as `>=1.2.3, <1.8`."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " - Either `*` (meaning \"any\"), or one or more comma-separated comparators."] # [doc = ""] # [doc = " - A [`Comparator`] is an operator ([`Op`]) and a partial version, separated"] # [doc = "   by optional whitespace. For example `>=1.0.0` or `>=1.0`."] # [doc = ""] # [doc = " - Build metadata is syntactically permitted on the partial versions, but is"] # [doc = "   completely ignored, as it's never relevant to whether any comparator"] # [doc = "   matches a particular version."] # [doc = ""] # [doc = " - Whitespace is permitted around commas and around operators. Whitespace is"] # [doc = "   not permitted within a partial version, i.e. anywhere between the major"] # [doc = "   version number and its minor, patch, pre-release, or build metadata."] # [derive (Clone , Eq , PartialEq , Hash , Debug)] pub struct VersionReq { pub comparators : Vec < Comparator > , }
    };
}

VersionReq!();