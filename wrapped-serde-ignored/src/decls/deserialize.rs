macro_rules! deps {
    () => {
        Path!();
        Deserializer!();
    };
}

macro_rules! deserialize {
    () => {
        deps!();
        # [doc = " Entry point. See crate documentation for an example."] pub fn deserialize < 'de , D , F , T > (deserializer : D , mut callback : F) -> Result < T , D :: Error > where D : de :: Deserializer < 'de > , F : FnMut (Path) , T : Deserialize < 'de > , { T :: deserialize (Deserializer :: new (deserializer , & mut callback)) }
    };
}

deserialize!()