macro_rules! deps {
    () => {
        Entry!();
        Slab!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T > ops :: Index < usize > for Slab < T > { type Output = T ; # [track_caller] fn index (& self , key : usize) -> & T { match self . entries . get (key) { Some (Entry :: Occupied (v)) => v , _ => panic ! ("invalid key") , } } }
    };
}

impl_25!();