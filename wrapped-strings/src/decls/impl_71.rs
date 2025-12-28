macro_rules! deps {
    () => {
        HStringBuilder!();
        HSTRING!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < HStringBuilder > for HSTRING { fn from (value : HStringBuilder) -> Self { if let Some (header) = value . as_header () { unsafe { header . data . offset (header . len as isize) . write (0) } ; let result = Self (value . 0) ; core :: mem :: forget (value) ; result } else { Self :: new () } } }
    };
}

impl_71!()