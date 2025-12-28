macro_rules! deps {
    () => {
        MatchArm!();
        PatCx!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Clone for MatchArm < 'p , Cx > { fn clone (& self) -> Self { Self { pat : self . pat , has_guard : self . has_guard , arm_data : self . arm_data } } }
    };
}

impl_14!()