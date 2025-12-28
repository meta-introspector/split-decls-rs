macro_rules! deps {
    () => {
        ThreadSpawn!();
        CustomSpawn!();
        ThreadBuilder!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < F > ThreadSpawn for CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { private_impl ! { } # [inline] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { (self . 0) (thread) } }
    };
}

impl_106!()