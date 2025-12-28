macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (all (loom , test))] impl < T : Clear > Clear for crate :: sync :: alloc :: Track < T > { fn clear (& mut self) { self . get_mut () . clear () } }
    };
}

impl_68!()