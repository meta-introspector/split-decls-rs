macro_rules! deps {
    () => {
        Cx!();
        Stack!();
        StackEntry!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < X : Cx > Index < StackDepth > for Stack < X > { type Output = StackEntry < X > ; fn index (& self , index : StackDepth) -> & StackEntry < X > { & self . entries [index] } }
    };
}

impl_132!();