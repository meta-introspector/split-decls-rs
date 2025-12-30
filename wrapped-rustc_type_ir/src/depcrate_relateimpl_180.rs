// Generated macro for impl_180 (impl)
macro_rules! Depcrate_relateimpl_180 {
() => {
// Module: crate::relate
// Provides: {"impl_180"}
// Dependencies: {}
impl < I : Interner > Relate < I > for ty :: ExistentialProjection < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: ExistentialProjection < I > , b : ty :: ExistentialProjection < I > ,) -> RelateResult < I , ty :: ExistentialProjection < I > > { if a . def_id != b . def_id { Err (TypeError :: ProjectionMismatched ({ let a = a . def_id ; let b = b . def_id ; ExpectedFound :: new (a , b) })) } else { let term = relation . relate_with_variance (ty :: Invariant , VarianceDiagInfo :: default () , a . term , b . term ,) ? ; let args = relation . relate_with_variance (ty :: Invariant , VarianceDiagInfo :: default () , a . args , b . args ,) ? ; Ok (ty :: ExistentialProjection :: new_from_args (relation . cx () , a . def_id , args , term)) } } }
};
}
