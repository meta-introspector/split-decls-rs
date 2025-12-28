macro_rules! deps {
    () => {
        Const!();
        BoundConstness!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl BoundConstness { pub fn satisfies (self , goal : BoundConstness) -> bool { match (self , goal) { (BoundConstness :: Const , BoundConstness :: Const | BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Const) => false , } } pub fn as_str (self) -> & 'static str { match self { Self :: Const => "const" , Self :: Maybe => "[const]" , } } }
    };
}

impl_399!()