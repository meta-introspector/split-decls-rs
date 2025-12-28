macro_rules! deps {
    () => {
        Constructor!();
        IntRange!();
        Slice!();
        PatCx!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < Cx : PatCx > Clone for Constructor < Cx > { fn clone (& self) -> Self { match self { Constructor :: Struct => Constructor :: Struct , Constructor :: Variant (idx) => Constructor :: Variant (* idx) , Constructor :: Ref => Constructor :: Ref , Constructor :: Slice (slice) => Constructor :: Slice (* slice) , Constructor :: UnionField => Constructor :: UnionField , Constructor :: Bool (b) => Constructor :: Bool (* b) , Constructor :: IntRange (range) => Constructor :: IntRange (* range) , Constructor :: F16Range (lo , hi , end) => Constructor :: F16Range (* lo , * hi , * end) , Constructor :: F32Range (lo , hi , end) => Constructor :: F32Range (* lo , * hi , * end) , Constructor :: F64Range (lo , hi , end) => Constructor :: F64Range (* lo , * hi , * end) , Constructor :: F128Range (lo , hi , end) => Constructor :: F128Range (* lo , * hi , * end) , Constructor :: Str (value) => Constructor :: Str (value . clone ()) , Constructor :: DerefPattern (ty) => Constructor :: DerefPattern (ty . clone ()) , Constructor :: Opaque (inner) => Constructor :: Opaque (inner . clone ()) , Constructor :: Or => Constructor :: Or , Constructor :: Never => Constructor :: Never , Constructor :: Wildcard => Constructor :: Wildcard , Constructor :: NonExhaustive => Constructor :: NonExhaustive , Constructor :: Hidden => Constructor :: Hidden , Constructor :: Missing => Constructor :: Missing , Constructor :: PrivateUninhabited => Constructor :: PrivateUninhabited , } } }
    };
}

impl_18!()