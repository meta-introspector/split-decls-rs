macro_rules! deps {
    () => {
        MakeVisitor!();
        RecordFields!();
        VisitOutput!();
    };
}

macro_rules! MakeOutput {
    () => {
        deps!();
        # [doc = " Extension trait implemented for all `MakeVisitor` implementations that"] # [doc = " produce a visitor implementing `VisitOutput`."] pub trait MakeOutput < T , Out > where Self : MakeVisitor < T > + crate :: sealed :: Sealed < (T , Out) > , Self :: Visitor : VisitOutput < Out > , { # [doc = " Visits all fields in `fields` with a new visitor constructed from"] # [doc = " `target`."] fn visit_with < F > (& self , target : T , fields : & F) -> Out where F : RecordFields , { self . make_visitor (target) . visit (fields) } }
    };
}

MakeOutput!()