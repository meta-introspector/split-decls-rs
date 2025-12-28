macro_rules! deps {
    () => {
        BlockAndExtension!();
        BlockAnd!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl BlockAndExtension for BasicBlock { fn and < T > (self , v : T) -> BlockAnd < T > { BlockAnd (self , v) } fn unit (self) -> BlockAnd < () > { BlockAnd (self , ()) } }
    };
}

impl_22!();