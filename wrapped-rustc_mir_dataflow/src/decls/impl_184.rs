macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl MoveOutIndex { pub fn move_path_index (self , move_data : & MoveData < '_ >) -> MovePathIndex { move_data . moves [self] . path } }
    };
}

impl_184!();