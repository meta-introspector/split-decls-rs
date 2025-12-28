macro_rules! deps {
    () => {
        Interner!();
        Variance!();
        RelateResult!();
        Relate!();
        Ty!();
        Binder!();
        DefId!();
        Region!();
        VarianceDiagInfo!();
        Const!();
        GenericArgs!();
    };
}

macro_rules! TypeRelation {
    () => {
        deps!();
        pub trait TypeRelation < I : Interner > : Sized { fn cx (& self) -> I ; # [doc = " Generic relation routine suitable for most anything."] fn relate < T : Relate < I > > (& mut self , a : T , b : T) -> RelateResult < I , T > { Relate :: relate (self , a , b) } # [doc = " Relate the two args for the given item. The default"] # [doc = " is to look up the variance for the item and proceed"] # [doc = " accordingly."] # [instrument (skip (self) , level = "trace")] fn relate_item_args (& mut self , item_def_id : I :: DefId , a_arg : I :: GenericArgs , b_arg : I :: GenericArgs ,) -> RelateResult < I , I :: GenericArgs > { let cx = self . cx () ; let opt_variances = cx . variances_of (item_def_id) ; relate_args_with_variances (self , item_def_id , opt_variances , a_arg , b_arg , true) } # [doc = " Switch variance for the purpose of relating `a` and `b`."] fn relate_with_variance < T : Relate < I > > (& mut self , variance : ty :: Variance , info : VarianceDiagInfo < I > , a : T , b : T ,) -> RelateResult < I , T > ; fn tys (& mut self , a : I :: Ty , b : I :: Ty) -> RelateResult < I , I :: Ty > ; fn regions (& mut self , a : I :: Region , b : I :: Region) -> RelateResult < I , I :: Region > ; fn consts (& mut self , a : I :: Const , b : I :: Const) -> RelateResult < I , I :: Const > ; fn binders < T > (& mut self , a : ty :: Binder < I , T > , b : ty :: Binder < I , T > ,) -> RelateResult < I , ty :: Binder < I , T > > where T : Relate < I > ; }
    };
}

TypeRelation!();