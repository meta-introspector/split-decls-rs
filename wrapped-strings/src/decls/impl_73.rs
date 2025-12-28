macro_rules! deps {
    () => {
        HStringBuilder!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl core :: ops :: DerefMut for HStringBuilder { fn deref_mut (& mut self) -> & mut [u16] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts_mut (header . data , header . len as usize) } } else { & mut [] } } }
    };
}

impl_73!()