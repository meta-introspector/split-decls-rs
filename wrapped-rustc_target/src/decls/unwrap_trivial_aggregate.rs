macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! unwrap_trivial_aggregate {
    () => {
        deps!();
        fn unwrap_trivial_aggregate < 'a , Ty , C > (cx : & C , val : & mut ArgAbi < 'a , Ty >) -> bool where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if val . layout . is_aggregate () { if let Some (unit) = val . layout . homogeneous_aggregate (cx) . ok () . and_then (| ha | ha . unit ()) { let size = val . layout . size ; if unit . size == size { val . cast_to (unit) ; return true ; } } } false }
    };
}

unwrap_trivial_aggregate!()