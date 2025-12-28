macro_rules! deps {
    () => {
        Scan!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl PartialEq for Scan { fn eq (& self , other : & Self) -> bool { * self as u8 == * other as u8 } }
    };
}

impl_286!()