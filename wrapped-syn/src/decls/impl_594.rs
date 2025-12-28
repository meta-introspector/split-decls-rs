macro_rules! deps {
    () => {
        Precedence!();
    };
}

macro_rules! impl_594 {
    () => {
        deps!();
        impl PartialEq for Precedence { fn eq (& self , other : & Self) -> bool { * self as u8 == * other as u8 } }
    };
}

impl_594!()