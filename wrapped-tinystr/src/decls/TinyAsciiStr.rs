macro_rules! deps {
    () => {
        AsciiByte!();
    };
}

macro_rules! TinyAsciiStr {
    () => {
        deps!();
        # [repr (transparent)] # [derive (PartialEq , Eq , Ord , PartialOrd , Copy , Clone , Hash)] pub struct TinyAsciiStr < const N : usize > { bytes : [AsciiByte ; N] , }
    };
}

TinyAsciiStr!()