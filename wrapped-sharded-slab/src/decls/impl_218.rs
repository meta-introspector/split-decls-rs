macro_rules! deps {
    () => {
        DontDropMe!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl PartialEq for DontDropMe { fn eq (& self , other : & DontDropMe) -> bool { self . 0 . eq (& other . 0) } }
    };
}

impl_218!();