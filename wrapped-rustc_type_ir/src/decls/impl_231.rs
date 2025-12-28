macro_rules! deps {
    () => {
        Const!();
        Binder!();
        TypeVisitable!();
        Ty!();
        ValidateBoundVars!();
        Region!();
        Interner!();
        TypeVisitor!();
        ConstKind!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < I : Interner > TypeVisitor < I > for ValidateBoundVars < I > { type Result = ControlFlow < () > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & Binder < I , T >) -> Self :: Result { self . binder_index . shift_in (1) ; let result = t . super_visit_with (self) ; self . binder_index . shift_out (1) ; result } fn visit_ty (& mut self , t : I :: Ty) -> Self :: Result { if t . outer_exclusive_binder () < self . binder_index || ! self . visited . insert ((self . binder_index , t)) { return ControlFlow :: Break (()) ; } match t . kind () { ty :: Bound (debruijn , bound_ty) if debruijn == self . binder_index => { let idx = bound_ty . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , t , self . bound_vars) ; } bound_ty . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => { } } ; t . super_visit_with (self) } fn visit_const (& mut self , c : I :: Const) -> Self :: Result { if c . outer_exclusive_binder () < self . binder_index { return ControlFlow :: Break (()) ; } match c . kind () { ty :: ConstKind :: Bound (debruijn , bound_const) if debruijn == self . binder_index => { let idx = bound_const . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , c , self . bound_vars) ; } bound_const . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => { } } ; c . super_visit_with (self) } fn visit_region (& mut self , r : I :: Region) -> Self :: Result { match r . kind () { ty :: ReBound (index , br) if index == self . binder_index => { let idx = br . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , r , self . bound_vars) ; } br . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => () , } ; ControlFlow :: Continue (()) } }
    };
}

impl_231!();