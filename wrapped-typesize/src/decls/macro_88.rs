macro_rules! macro_88 {
    () => {
        create_ref ! (# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Ref <'a , T : ? Sized > (pub &'a T)) ;
    };
}

macro_88!()