macro_rules! deps {
    () => {
        LitFloatRepr!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for LitFloatRepr { fn clone (& self) -> Self { LitFloatRepr { token : self . token . clone () , digits : self . digits . clone () , suffix : self . suffix . clone () , } } }
    };
}

impl_437!();