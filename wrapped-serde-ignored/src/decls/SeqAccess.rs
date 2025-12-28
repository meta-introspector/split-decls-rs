macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! SeqAccess {
    () => {
        deps!();
        # [doc = " Seq visitor that tracks the index of its elements."] struct SeqAccess < 'a , 'b , X , F : 'b > { delegate : X , callback : & 'b mut F , path : & 'a Path < 'a > , index : usize , }
    };
}

SeqAccess!();