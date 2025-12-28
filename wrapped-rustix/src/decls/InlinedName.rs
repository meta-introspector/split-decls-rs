macro_rules! InlinedName {
    () => {
        # [doc = " The inlined interface name."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] pub struct InlinedName { len : usize , name : [u8 ; 16] , }
    };
}

InlinedName!();