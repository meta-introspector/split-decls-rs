macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Deref for HSTRING { type Target = [u16] ; fn deref (& self) -> & [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts (header . data , header . len as usize) } } else { const EMPTY : [u16 ; 1] = [0] ; & EMPTY [.. 0] } } }
    };
}

impl_21!()