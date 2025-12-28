macro_rules! deps {
    () => {
        HStringBuilder!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl core :: ops :: Deref for HStringBuilder { type Target = [u16] ; fn deref (& self) -> & [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts (header . data , header . len as usize) } } else { & [] } } }
    };
}

impl_72!();