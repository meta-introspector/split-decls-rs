macro_rules! deps {
    () => {
        TyConst!();
        GenericArgKind!();
        Ty!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl GenericArgKind { # [doc = " Panic if this generic argument is not a type, otherwise"] # [doc = " return the type."] # [track_caller] pub fn expect_ty (& self) -> & Ty { match self { GenericArgKind :: Type (ty) => ty , _ => panic ! ("{self:?}") , } } # [doc = " Panic if this generic argument is not a const, otherwise"] # [doc = " return the const."] # [track_caller] pub fn expect_const (& self) -> & TyConst { match self { GenericArgKind :: Const (c) => c , _ => panic ! ("{self:?}") , } } # [doc = " Return the generic argument type if applicable, otherwise return `None`."] pub fn ty (& self) -> Option < & Ty > { match self { GenericArgKind :: Type (ty) => Some (ty) , _ => None , } } }
    };
}

impl_381!();