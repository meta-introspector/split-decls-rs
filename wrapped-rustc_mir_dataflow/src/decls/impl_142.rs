macro_rules! deps {
    () => {
        MoveData!();
        EverInitializedPlaces!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a , 'tcx > EverInitializedPlaces < 'a , 'tcx > { pub fn new (body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { EverInitializedPlaces { body , move_data } } }
    };
}

impl_142!();