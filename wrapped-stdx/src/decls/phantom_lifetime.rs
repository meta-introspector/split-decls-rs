macro_rules! deps {
    () => {
        Variance!();
    };
}

macro_rules! phantom_lifetime {
    () => {
        deps!();
        macro_rules ! phantom_lifetime { ($ ($ (# [$ attr : meta]) * pub struct $ name : ident <$ lt : lifetime > ($ ($ inner : tt) *) ;) *) => { $ ($ (# [$ attr]) * # [derive (Default , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct $ name <$ lt > ($ ($ inner) *) ; impl $ name <'_ > { # [doc = " Constructs a new instance of the variance marker."] pub const fn new () -> Self { Self (first_token ! ($ ($ inner) *) (PhantomData)) } } impl self :: sealed :: Sealed for $ name <'_ > { const VALUE : Self = Self :: new () ; } impl Variance for $ name <'_ > { } impl fmt :: Debug for $ name <'_ > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "{}" , stringify ! ($ name)) } }) * } ; }
    };
}

phantom_lifetime!()