// Generated macro for impl_1512 (impl)
macro_rules! Depcrate_typesimpl_1512 {
() => {
// Module: crate::types
// Provides: {"impl_1512"}
// Dependencies: {}
impl Rewrite for ast :: AssocItemConstraint { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { use ast :: AssocItemConstraintKind :: { Bound , Equality } ; let mut result = String :: with_capacity (128) ; result . push_str (rewrite_ident (context , self . ident)) ; if let Some (ref gen_args) = self . gen_args { let budget = shape . width . checked_sub (result . len ()) . max_width_error (shape . width , self . span) ? ; let shape = Shape :: legacy (budget , shape . indent + result . len ()) ; let gen_str = rewrite_generic_args (gen_args , context , shape , gen_args . span ()) ? ; result . push_str (& gen_str) ; } let infix = match (& self . kind , context . config . type_punctuation_density ()) { (Bound { .. } , _) => ": " , (Equality { .. } , TypeDensity :: Wide) => " = " , (Equality { .. } , TypeDensity :: Compressed) => "=" , } ; result . push_str (infix) ; let budget = shape . width . checked_sub (result . len ()) . max_width_error (shape . width , self . span) ? ; let shape = Shape :: legacy (budget , shape . indent + result . len ()) ; let rewrite = self . kind . rewrite_result (context , shape) ? ; result . push_str (& rewrite) ; Ok (result) } }
};
}
