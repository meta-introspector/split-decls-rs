macro_rules! macro_103 {
    () => {
        declare_lint ! { # [doc = " The `deprecated_where_clause_location` lint detects when a where clause in front of the equals"] # [doc = " in an associated type."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " trait Trait {"] # [doc = "   type Assoc<'a> where Self: 'a;"] # [doc = " }"] # [doc = ""] # [doc = " impl Trait for () {"] # [doc = "   type Assoc<'a> where Self: 'a = ();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred location for where clauses on associated types"] # [doc = " is after the type. However, for most of generic associated types development,"] # [doc = " it was only accepted before the equals. To provide a transition period and"] # [doc = " further evaluate this change, both are currently accepted. At some point in"] # [doc = " the future, this may be disallowed at an edition boundary; but, that is"] # [doc = " undecided currently."] pub DEPRECATED_WHERE_CLAUSE_LOCATION , Warn , "deprecated where clause location" }
    };
}

macro_103!();