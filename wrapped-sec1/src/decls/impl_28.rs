macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < Tag > for u8 { fn from (tag : Tag) -> u8 { tag as u8 } }
    };
}

impl_28!();