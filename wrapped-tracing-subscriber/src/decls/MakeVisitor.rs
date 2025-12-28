macro_rules! MakeVisitor {
    () => {
        # [doc = " Creates new [visitors]."] # [doc = ""] # [doc = " A type implementing `MakeVisitor` represents a composable factory for types"] # [doc = " implementing the [`Visit` trait][visitors]. The `MakeVisitor` trait defines"] # [doc = " a single function, `make_visitor`, which takes in a `T`-typed `target` and"] # [doc = " returns a type implementing `Visit` configured for that target. A target may"] # [doc = " be a string, output stream, or data structure that the visitor will record"] # [doc = " data to, configuration variables that determine the visitor's behavior, or"] # [doc = " `()` when no input is required to produce a visitor."] # [doc = ""] # [doc = " [visitors]: tracing_core::field::Visit"] pub trait MakeVisitor < T > { # [doc = " The visitor type produced by this `MakeVisitor`."] type Visitor : Visit ; # [doc = " Make a new visitor for the provided `target`."] fn make_visitor (& self , target : T) -> Self :: Visitor ; }
    };
}

MakeVisitor!();