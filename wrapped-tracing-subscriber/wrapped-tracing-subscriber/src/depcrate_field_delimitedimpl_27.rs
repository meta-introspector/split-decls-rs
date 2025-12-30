// Generated macro for impl_27 (impl)
macro_rules! Depcrate_field_delimitedimpl_27 {
() => {
// Module: crate::field::delimited
// Provides: {"impl_27"}
// Dependencies: {}
impl < D , V , T > MakeVisitor < T > for Delimited < D , V > where D : AsRef < str > + Clone , V : MakeVisitor < T > , V :: Visitor : VisitFmt , { type Visitor = VisitDelimited < D , V :: Visitor > ; fn make_visitor (& self , target : T) -> Self :: Visitor { let inner = self . inner . make_visitor (target) ; VisitDelimited :: new (self . delimiter . clone () , inner) } }
};
}
