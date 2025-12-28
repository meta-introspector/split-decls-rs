macro_rules! deps {
    () => {
        EdgeHeader!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl EdgeHeader { # [inline] fn start (self) -> usize { self . repr >> DEP_NODE_WIDTH_BITS } # [inline] fn bytes_per_index (self) -> usize { (self . repr & mask (DEP_NODE_WIDTH_BITS)) + 1 } # [inline] fn mask (self) -> u32 { mask (self . bytes_per_index () * 8) as u32 } }
    };
}

impl_82!()