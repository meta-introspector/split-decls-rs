macro_rules! deps {
    () => {
        Index!();
        Array!();
        Value!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Index for usize { fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > { match * val { Value :: Array (ref a) => a . get (* self) , _ => None , } } fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > { match * val { Value :: Array (ref mut a) => a . get_mut (* self) , _ => None , } } }
    };
}

impl_79!();