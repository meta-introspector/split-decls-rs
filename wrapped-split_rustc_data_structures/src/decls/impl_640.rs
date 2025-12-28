macro_rules! deps {
    () => {
        UnordItems!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'a , T : Clone + 'a , I : Iterator < Item = & 'a T > > UnordItems < & 'a T , I > { # [inline] pub fn cloned (self) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . cloned ()) } }
    };
}

impl_640!();