macro_rules! Tlv {
    () => {
        # [derive (Copy , Clone)] pub (crate) struct Tlv (pub (crate) * const ()) ;
    };
}

Tlv!()