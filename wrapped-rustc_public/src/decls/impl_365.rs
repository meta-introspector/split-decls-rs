macro_rules! deps {
    () => {
        AdtKind!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl AdtKind { pub fn is_enum (& self) -> bool { matches ! (self , AdtKind :: Enum) } pub fn is_struct (& self) -> bool { matches ! (self , AdtKind :: Struct) } pub fn is_union (& self) -> bool { matches ! (self , AdtKind :: Union) } }
    };
}

impl_365!();