macro_rules! IntoKind {
    () => {
        pub trait IntoKind { type Kind ; fn kind (self) -> Self :: Kind ; }
    };
}

IntoKind!()