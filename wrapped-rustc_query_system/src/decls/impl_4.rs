macro_rules! deps {
    () => {
        DepContext!();
        Value!();
        WithDepNode!();
        Cache!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < Key : Eq + Hash , Value : Clone > Cache < Key , Value > { pub fn get < Tcx : DepContext > (& self , key : & Key , tcx : Tcx) -> Option < Value > { Some (self . hashmap . borrow () . get (key) ? . get (tcx)) } pub fn insert (& self , key : Key , dep_node : DepNodeIndex , value : Value) { self . hashmap . borrow_mut () . insert (key , WithDepNode :: new (dep_node , value)) ; } }
    };
}

impl_4!();