macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'computed > Drop for Label < 'computed > { fn drop (& mut self) { # [cfg (feature = "alloc")] { if let Some (owned) = self . backing_field_owned { drop (unsafe { Box :: from_raw (owned) }) ; } } } }
    };
}

impl_13!();