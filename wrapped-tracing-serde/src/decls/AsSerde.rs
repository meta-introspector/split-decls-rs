macro_rules! AsSerde {
    () => {
        pub trait AsSerde < 'a > : self :: sealed :: Sealed { type Serializable : serde :: Serialize + 'a ; # [doc = " `as_serde` borrows a `tracing` value and returns the serialized value."] fn as_serde (& 'a self) -> Self :: Serializable ; }
    };
}

AsSerde!()