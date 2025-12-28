macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl From < & str > for ThinVec < u8 > { # [doc = " Allocate a `ThinVec<u8>` and fill it with a UTF-8 string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " assert_eq!(ThinVec::from(\"123\"), thin_vec![b'1', b'2', b'3']);"] # [doc = " ```"] fn from (s : & str) -> ThinVec < u8 > { From :: from (s . as_bytes ()) } }
    };
}

impl_58!();