macro_rules! deps {
    () => {
        ChangeLt!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl syn :: visit_mut :: VisitMut for ChangeLt < '_ > { fn visit_lifetime_mut (& mut self , i : & mut syn :: Lifetime) { if self . from . map (| f | i . ident == f) . unwrap_or (true) { i . ident = syn :: Ident :: new (& self . to , i . ident . span ()) ; } } }
    };
}

impl_106!()