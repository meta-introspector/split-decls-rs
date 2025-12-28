macro_rules! deps {
    () => {
        StateData!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < V : Clone > Clone for StateData < V > { fn clone (& self) -> Self { StateData { bottom : self . bottom . clone () , map : self . map . clone () } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) } }
    };
}

impl_245!();