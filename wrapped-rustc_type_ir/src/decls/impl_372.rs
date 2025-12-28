macro_rules! deps {
    () => {
        UnevaluatedConst!();
        AliasTermKind!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl AliasTermKind { pub fn descr (self) -> & 'static str { match self { AliasTermKind :: ProjectionTy => "associated type" , AliasTermKind :: ProjectionConst => "associated const" , AliasTermKind :: InherentTy => "inherent associated type" , AliasTermKind :: InherentConst => "inherent associated const" , AliasTermKind :: OpaqueTy => "opaque type" , AliasTermKind :: FreeTy => "type alias" , AliasTermKind :: FreeConst => "unevaluated constant" , AliasTermKind :: UnevaluatedConst => "unevaluated constant" , } } pub fn is_type (self) -> bool { match self { AliasTermKind :: ProjectionTy | AliasTermKind :: InherentTy | AliasTermKind :: OpaqueTy | AliasTermKind :: FreeTy => true , AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst | AliasTermKind :: InherentConst | AliasTermKind :: FreeConst => false , } } }
    };
}

impl_372!()