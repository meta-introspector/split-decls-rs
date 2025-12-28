macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! TomlString {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub struct TomlString < 's > { decoded : & 's str , encoding : Encoding , newline : bool , }
    };
}

TomlString!();