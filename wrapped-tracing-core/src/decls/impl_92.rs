macro_rules! deps {
    () => {
        Registrar!();
        Dispatch!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Registrar { pub (crate) fn upgrade (& self) -> Option < Dispatch > { self . 0 . upgrade () . map (| subscriber | Dispatch { subscriber }) } }
    };
}

impl_92!();