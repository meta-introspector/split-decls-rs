macro_rules! define_float_conversion {
    () => {
        macro_rules ! define_float_conversion { ($ ty : ty , $ bits : ident , $ bytes : expr , $ mod : ident) => { mod $ mod { use super ::*; define_float_conversion ! ($ ty , $ bits , $ bytes , from_be_bytes , to_be_bytes) ; define_float_conversion ! ($ ty , $ bits , $ bytes , from_le_bytes , to_le_bytes) ; } } ; ($ ty : ty , $ bits : ident , $ bytes : expr , $ from : ident , $ to : ident) => { # [allow (clippy :: unnecessary_transmutes)] pub (crate) const fn $ from (bytes : [u8 ; $ bytes]) -> $ ty { transmute ! ($ bits ::$ from (bytes)) } pub (crate) const fn $ to (f : $ ty) -> [u8 ; $ bytes] { # [allow (clippy :: unnecessary_transmutes)] let bits : $ bits = transmute ! (f) ; bits .$ to () } } ; }
    };
}

define_float_conversion!()