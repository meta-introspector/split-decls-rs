macro_rules! deps {
    () => {
        BlockAnd!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl BlockAnd < () > { # [doc = " Unpacks `BlockAnd<()>` into a [`BasicBlock`]."] # [must_use] fn into_block (self) -> BasicBlock { let Self (block , ()) = self ; block } }
    };
}

impl_20!()