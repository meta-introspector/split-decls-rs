macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! Deserializer {
    () => {
        deps!();
        # [doc = " Deserialization for TOML [documents][crate::DocumentMut]."] pub struct Deserializer < S = String > { root : crate :: Item , raw : Option < S > , }
    };
}

Deserializer!();