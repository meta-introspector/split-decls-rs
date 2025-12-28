macro_rules! deps {
    () => {
        LitIntRepr!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for LitIntRepr { fn clone (& self) -> Self { LitIntRepr { token : self . token . clone () , digits : self . digits . clone () , suffix : self . suffix . clone () , } } }
    };
}

impl_436!()