macro_rules! deps {
    () => {
        CycleHead!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Clone for CycleHead { fn clone (& self) -> Self { Self { database_key_index : self . database_key_index , iteration_count : self . iteration_count . load () . into () , removed : self . removed . load (Ordering :: Relaxed) . into () , } } }
    };
}

impl_44!()