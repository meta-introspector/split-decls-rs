macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! TomlKey {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub struct TomlKey < 's > { decoded : & 's str , encoding : Option < Encoding > , }
    };
}

TomlKey!();