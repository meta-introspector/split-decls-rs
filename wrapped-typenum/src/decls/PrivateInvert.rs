macro_rules! PrivateInvert {
    () => {
        # [doc = " Doubly private! Called by invert to make the magic happen once its done the first step."] # [doc = " The Rhs is what we've got so far."] pub trait PrivateInvert < Rhs > { type Output ; fn private_invert (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateInvert!()