macro_rules! Byte {
    () => {
        # [doc = " A range of byte values (including an uninit byte value)."] # [derive (Hash , Eq , PartialEq , Ord , PartialOrd , Clone , Copy)] pub (crate) struct Byte { pub (crate) start : u16 , pub (crate) end : u16 , }
    };
}

Byte!()