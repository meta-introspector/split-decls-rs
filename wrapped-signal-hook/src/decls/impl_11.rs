macro_rules! deps {
    () => {
        DeliveryState!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Drop for DeliveryState { fn drop (& mut self) { let lock = self . registered_signal_ids . lock () . unwrap () ; for id in lock . iter () . filter_map (| s | * s) { crate :: low_level :: unregister (id) ; } } }
    };
}

impl_11!()