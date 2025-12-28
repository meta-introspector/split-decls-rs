macro_rules! deps {
    () => {
        Runtime!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Runtime { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { fmt . debug_struct ("Runtime") . field ("revisions" , & self . revisions) . field ("revision_canceled" , & self . revision_canceled) . field ("dependency_graph" , & self . dependency_graph) . finish () } }
    };
}

impl_279!();