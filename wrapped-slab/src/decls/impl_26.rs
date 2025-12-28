macro_rules! deps {
    () => {
        Entry!();
        Slab!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T > ops :: IndexMut < usize > for Slab < T > { # [track_caller] fn index_mut (& mut self , key : usize) -> & mut T { match self . entries . get_mut (key) { Some (& mut Entry :: Occupied (ref mut v)) => v , _ => panic ! ("invalid key") , } } }
    };
}

impl_26!()