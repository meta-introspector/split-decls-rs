macro_rules! deps {
    () => {
        FnSig!();
        Binder!();
        Interner!();
        EarlyBinder!();
        DefId!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < I : Interner , T > EarlyBinder < I , T > { pub fn bind (value : T) -> EarlyBinder < I , T > { EarlyBinder { value , _tcx : PhantomData } } pub fn as_ref (& self) -> EarlyBinder < I , & T > { EarlyBinder { value : & self . value , _tcx : PhantomData } } pub fn map_bound_ref < F , U > (& self , f : F) -> EarlyBinder < I , U > where F : FnOnce (& T) -> U , { self . as_ref () . map_bound (f) } pub fn map_bound < F , U > (self , f : F) -> EarlyBinder < I , U > where F : FnOnce (T) -> U , { let value = f (self . value) ; EarlyBinder { value , _tcx : PhantomData } } pub fn try_map_bound < F , U , E > (self , f : F) -> Result < EarlyBinder < I , U > , E > where F : FnOnce (T) -> Result < U , E > , { let value = f (self . value) ? ; Ok (EarlyBinder { value , _tcx : PhantomData }) } pub fn rebind < U > (& self , value : U) -> EarlyBinder < I , U > { EarlyBinder { value , _tcx : PhantomData } } # [doc = " Skips the binder and returns the \"bound\" value. Accessing generic args"] # [doc = " in the returned value is generally incorrect."] # [doc = ""] # [doc = " Please read <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html>"] # [doc = " before using this function."] # [doc = ""] # [doc = " Only use this to extract data that does not depend on generic parameters, e.g."] # [doc = " to get the `DefId` of the inner value or the number of arguments ofan `FnSig`,"] # [doc = " or while making sure to only pass the value to functions which are explicitly"] # [doc = " set up to handle these uninstantiated generic parameters."] # [doc = ""] # [doc = " To skip the binder on `x: &EarlyBinder<I, T>` to obtain `&T`, leverage"] # [doc = " [`EarlyBinder::as_ref`](EarlyBinder::as_ref): `x.as_ref().skip_binder()`."] # [doc = ""] # [doc = " See also [`Binder::skip_binder`](Binder::skip_binder), which is"] # [doc = " the analogous operation on [`Binder`]."] pub fn skip_binder (self) -> T { self . value } }
    };
}

impl_236!()