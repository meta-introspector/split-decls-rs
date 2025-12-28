macro_rules! SpannedTypeVisitor {
    () => {
        pub trait SpannedTypeVisitor < 'tcx > { type Result : VisitorResult = () ; fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) -> Self :: Result ; }
    };
}

SpannedTypeVisitor!()