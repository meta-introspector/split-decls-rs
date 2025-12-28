macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < V : Clone > Clone for State < V > { fn clone (& self) -> Self { match self { Self :: Reachable (x) => Self :: Reachable (x . clone ()) , Self :: Unreachable => Self :: Unreachable , } } fn clone_from (& mut self , source : & Self) { match (& mut * self , source) { (Self :: Reachable (x) , Self :: Reachable (y)) => { x . clone_from (& y) ; } _ => * self = source . clone () , } } }
    };
}

impl_248!();