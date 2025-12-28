macro_rules! impl_492 {
    () => {
        impl RelocModel { pub const fn desc_symbol (& self) -> Symbol { match * self { RelocModel :: Static => kw :: Static , RelocModel :: Pic => sym :: pic , RelocModel :: Pie => sym :: pie , RelocModel :: DynamicNoPic => sym :: dynamic_no_pic , RelocModel :: Ropi => sym :: ropi , RelocModel :: Rwpi => sym :: rwpi , RelocModel :: RopiRwpi => sym :: ropi_rwpi , } } pub const fn all () -> [Symbol ; 7] { [RelocModel :: Static . desc_symbol () , RelocModel :: Pic . desc_symbol () , RelocModel :: Pie . desc_symbol () , RelocModel :: DynamicNoPic . desc_symbol () , RelocModel :: Ropi . desc_symbol () , RelocModel :: Rwpi . desc_symbol () , RelocModel :: RopiRwpi . desc_symbol () ,] } }
    };
}

impl_492!();