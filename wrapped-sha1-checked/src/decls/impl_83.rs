macro_rules! deps {
    () => {
        Sha1!();
        CollisionResult!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl CollisionResult { # [doc = " Returns the output hash."] pub fn hash (& self) -> & Output < Sha1 > { match self { CollisionResult :: Ok (s) => s , CollisionResult :: Mitigated (s) => s , CollisionResult :: Collision (s) => s , } } # [doc = " Returns if there was a collision"] pub fn has_collision (& self) -> bool { ! matches ! (self , CollisionResult :: Ok (_)) } }
    };
}

impl_83!()