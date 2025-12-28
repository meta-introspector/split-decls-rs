macro_rules! deps {
    () => {
        Binder!();
        BoundVariableKind!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl < T > Binder < T > { # [doc = " Create a new binder with the given bound vars."] pub fn bind_with_vars (value : T , bound_vars : Vec < BoundVariableKind >) -> Self { Binder { value , bound_vars } } # [doc = " Create a new binder with no bounded variable."] pub fn dummy (value : T) -> Self { Binder { value , bound_vars : vec ! [] } } pub fn skip_binder (self) -> T { self . value } pub fn map_bound_ref < F , U > (& self , f : F) -> Binder < U > where F : FnOnce (& T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars : bound_vars . clone () } } pub fn map_bound < F , U > (self , f : F) -> Binder < U > where F : FnOnce (T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars } } }
    };
}

impl_392!();