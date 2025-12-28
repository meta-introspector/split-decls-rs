macro_rules! deps {
    () => {
        SalsaStructAllowedOptions!();
        Options!();
        SalsaField!();
    };
}

macro_rules! SalsaStruct {
    () => {
        deps!();
        pub (crate) struct SalsaStruct < 's , A : SalsaStructAllowedOptions > { struct_item : & 's syn :: ItemStruct , args : & 's Options < A > , fields : Vec < SalsaField < 's > > , }
    };
}

SalsaStruct!()