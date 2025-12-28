macro_rules! deps {
    () => {
        UnordItems!();
        UnordBag!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl < V > UnordBag < V > { # [inline] pub fn new () -> Self { Self { inner : Default :: default () } } # [inline] pub fn len (& self) -> usize { self . inner . len () } # [inline] pub fn push (& mut self , v : V) { self . inner . push (v) ; } # [inline] pub fn items (& self) -> UnordItems < & V , impl Iterator < Item = & V > > { UnordItems (self . inner . iter ()) } # [inline] pub fn into_items (self) -> UnordItems < V , impl Iterator < Item = V > > { UnordItems (self . inner . into_iter ()) } }
    };
}

impl_665!()