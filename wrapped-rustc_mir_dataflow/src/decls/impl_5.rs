macro_rules! deps {
    () => {
        DropFlagState!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl DropFlagState { pub fn value (self) -> bool { match self { DropFlagState :: Present => true , DropFlagState :: Absent => false , } } }
    };
}

impl_5!()