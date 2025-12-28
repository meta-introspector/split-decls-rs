macro_rules! PrivateSetBit {
    () => {
        pub trait PrivateSetBit < I , B > { type Output ; fn private_set_bit (self , _ : I , _ : B) -> Self :: Output ; }
    };
}

PrivateSetBit!()