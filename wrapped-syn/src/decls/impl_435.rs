macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for LitRepr { fn clone (& self) -> Self { LitRepr { token : self . token . clone () , suffix : self . suffix . clone () , } } }
    };
}

impl_435!()