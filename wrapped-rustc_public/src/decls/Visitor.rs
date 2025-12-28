macro_rules! deps {
    () => {
        TyConst!();
        Ty!();
        Region!();
    };
}

macro_rules! Visitor {
    () => {
        deps!();
        pub trait Visitor : Sized { type Break ; fn visit_ty (& mut self , ty : & Ty) -> ControlFlow < Self :: Break > { ty . super_visit (self) } fn visit_const (& mut self , c : & TyConst) -> ControlFlow < Self :: Break > { c . super_visit (self) } fn visit_reg (& mut self , reg : & Region) -> ControlFlow < Self :: Break > { reg . super_visit (self) } }
    };
}

Visitor!()