macro_rules! deps {
    () => {
        ValueConst!();
        PlaceholderConst!();
        ExprConst!();
        Interner!();
        UnevaluatedConst!();
        InferConst!();
    };
}

macro_rules! ConstKind {
    () => {
        deps!();
        # [doc = " Represents a constant in Rust."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum ConstKind < I : Interner > { # [doc = " A const generic parameter."] Param (I :: ParamConst) , # [doc = " Infer the value of the const."] Infer (InferConst) , # [doc = " Bound const variable, used only when preparing a trait query."] Bound (DebruijnIndex , I :: BoundConst) , # [doc = " A placeholder const - universally quantified higher-ranked const."] Placeholder (I :: PlaceholderConst) , # [doc = " An unnormalized const item such as an anon const or assoc const or free const item."] # [doc = " Right now anything other than anon consts does not actually work properly but this"] # [doc = " should"] Unevaluated (ty :: UnevaluatedConst < I >) , # [doc = " Used to hold computed value."] Value (I :: ValueConst) , # [doc = " A placeholder for a const which could not be computed; this is"] # [doc = " propagated to avoid useless error messages."] Error (I :: ErrorGuaranteed) , # [doc = " Unevaluated non-const-item, used by `feature(generic_const_exprs)` to represent"] # [doc = " const arguments such as `N + 1` or `foo(N)`"] Expr (I :: ExprConst) , }
    };
}

ConstKind!();