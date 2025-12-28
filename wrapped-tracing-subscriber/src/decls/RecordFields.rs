macro_rules! deps {
    () => {
        RecordFieldsMarker!();
    };
}

macro_rules! RecordFields {
    () => {
        deps!();
        # [doc = " Extension trait implemented by types which can be recorded by a [visitor]."] # [doc = ""] # [doc = " This allows writing code that is generic over `tracing_core`'s"] # [doc = " [`span::Attributes`][attr], [`span::Record`][rec], and [`Event`]"] # [doc = " types. These types all provide inherent `record` methods that allow a"] # [doc = " visitor to record their fields, but there is no common trait representing this."] # [doc = ""] # [doc = " With `RecordFields`, we can write code like this:"] # [doc = " ```"] # [doc = " use tracing_core::field::Visit;"] # [doc = " # use tracing_core::field::Field;"] # [doc = " use tracing_subscriber::field::RecordFields;"] # [doc = ""] # [doc = " struct MyVisitor {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " # impl MyVisitor { fn new() -> Self { Self{} } }"] # [doc = " impl Visit for MyVisitor {"] # [doc = "     // ..."] # [doc = " # fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}"] # [doc = " }"] # [doc = ""] # [doc = " fn record_with_my_visitor<R>(r: R)"] # [doc = " where"] # [doc = "     R: RecordFields,"] # [doc = " {"] # [doc = "     let mut visitor = MyVisitor::new();"] # [doc = "     r.record(&mut visitor);"] # [doc = " }"] # [doc = " ```"] # [doc = " [visitor]: tracing_core::field::Visit"] # [doc = " [attr]: tracing_core::span::Attributes"] # [doc = " [rec]: tracing_core::span::Record"] pub trait RecordFields : crate :: sealed :: Sealed < RecordFieldsMarker > { # [doc = " Record all the fields in `self` with the provided `visitor`."] fn record (& self , visitor : & mut dyn Visit) ; }
    };
}

RecordFields!();