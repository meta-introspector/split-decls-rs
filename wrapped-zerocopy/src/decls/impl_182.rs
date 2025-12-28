macro_rules! deps {
    () => {
        Alignment!();
        ConvertError!();
        Validity!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < A : fmt :: Debug , S : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for ConvertError < A , S , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Alignment (e) => f . debug_tuple ("Alignment") . field (e) . finish () , Self :: Size (e) => f . debug_tuple ("Size") . field (e) . finish () , Self :: Validity (e) => f . debug_tuple ("Validity") . field (e) . finish () , } } }
    };
}

impl_182!();