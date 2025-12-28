macro_rules! deps {
    () => {
        InferConst!();
        Interner!();
        Flags!();
        Relate!();
        IntoKind!();
        ConstKind!();
        Term!();
        TypeSuperFoldable!();
        GenericArg!();
        PlaceholderConst!();
        TypeSuperVisitable!();
        ExprConst!();
        UnevaluatedConst!();
    };
}

macro_rules! Const {
    () => {
        deps!();
        pub trait Const < I : Interner < Const = Self > > : Copy + Debug + Hash + Eq + Into < I :: GenericArg > + Into < I :: Term > + IntoKind < Kind = ty :: ConstKind < I > > + TypeSuperVisitable < I > + TypeSuperFoldable < I > + Relate < I > + Flags { fn new_infer (interner : I , var : ty :: InferConst) -> Self ; fn new_var (interner : I , var : ty :: ConstVid) -> Self ; fn new_bound (interner : I , debruijn : ty :: DebruijnIndex , bound_const : I :: BoundConst) -> Self ; fn new_anon_bound (interner : I , debruijn : ty :: DebruijnIndex , var : ty :: BoundVar) -> Self ; fn new_placeholder (interner : I , param : I :: PlaceholderConst) -> Self ; fn new_unevaluated (interner : I , uv : ty :: UnevaluatedConst < I >) -> Self ; fn new_expr (interner : I , expr : I :: ExprConst) -> Self ; fn new_error (interner : I , guar : I :: ErrorGuaranteed) -> Self ; fn new_error_with_message (interner : I , msg : impl ToString) -> Self { Self :: new_error (interner , interner . delay_bug (msg)) } fn is_ct_var (self) -> bool { matches ! (self . kind () , ty :: ConstKind :: Infer (ty :: InferConst :: Var (_))) } fn is_ct_error (self) -> bool { matches ! (self . kind () , ty :: ConstKind :: Error (_)) } }
    };
}

Const!();