macro_rules! Version {
    () => {
        # [doc = " Version number: `major.minor.patch`, ignoring release channel."] # [derive (PartialEq , Eq , Copy , Clone , PartialOrd , Ord)] pub struct Version (u64) ;
    };
}

Version!()