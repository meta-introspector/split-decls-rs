macro_rules! macro_89 {
    () => {
        create_ref ! (# [derive (PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct RefMut <'a , T : ? Sized > (pub &'a mut T)) ;
    };
}

macro_89!();