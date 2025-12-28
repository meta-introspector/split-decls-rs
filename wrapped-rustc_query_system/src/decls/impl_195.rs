macro_rules! deps {
    () => {
        QueryCache!();
        Value!();
        SingleCache!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < V > QueryCache for SingleCache < V > where V : Copy , { type Key = () ; type Value = V ; # [inline (always)] fn lookup (& self , _key : & ()) -> Option < (V , DepNodeIndex) > { self . cache . get () . copied () } # [inline] fn complete (& self , _key : () , value : V , index : DepNodeIndex) { self . cache . set ((value , index)) . ok () ; } fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) { if let Some (value) = self . cache . get () { f (& () , & value . 0 , value . 1) } } }
    };
}

impl_195!();