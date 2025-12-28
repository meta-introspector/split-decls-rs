macro_rules! deps {
    () => {
        BlockAnd!();
    };
}

macro_rules! BlockAndExtension {
    () => {
        deps!();
        trait BlockAndExtension { fn and < T > (self , v : T) -> BlockAnd < T > ; fn unit (self) -> BlockAnd < () > ; }
    };
}

BlockAndExtension!();