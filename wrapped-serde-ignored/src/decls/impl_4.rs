macro_rules! deps {
    () => {
        Deserializer!();
        Path!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a , 'b , D , F > Deserializer < 'a , 'b , D , F > where F : FnMut (Path) , { pub fn new (de : D , callback : & 'b mut F) -> Self { Deserializer { de , callback , path : Path :: Root , } } }
    };
}

impl_4!()