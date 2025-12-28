macro_rules! deps {
    () => {
        Database!();
        DatabaseDownCaster!();
        ViewCaster!();
        DatabaseDownCasterSig!();
        Views!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl Views { pub (crate) fn new < Db : Database > () -> Self { let source_type_id = TypeId :: of :: < Db > () ; let view_casters = boxcar :: Vec :: new () ; view_casters . push (ViewCaster :: new :: < dyn Database > (| db | db . ptr . cast :: < Db > ())) ; Self { source_type_id , view_casters , } } # [doc = " Add a new downcaster to `dyn DbView`."] pub fn add < Concrete : 'static , DbView : ? Sized + Any > (& self , func : fn (NonNull < Concrete >) -> NonNull < DbView > ,) -> & DatabaseDownCaster < DbView > { assert_eq ! (self . source_type_id , TypeId :: of ::< Concrete > () , "mismatched source type") ; let target_type_id = TypeId :: of :: < DbView > () ; if let Some ((_ , caster)) = self . view_casters . iter () . find (| (_ , u) | u . target_type_id == target_type_id) { return unsafe { & * (& raw const * caster) . cast :: < DatabaseDownCaster < DbView > > () } ; } let caster = unsafe { mem :: transmute :: < fn (NonNull < Concrete >) -> NonNull < DbView > , DatabaseDownCasterSig < DbView > > (func ,) } ; let caster = ViewCaster :: new :: < DbView > (caster) ; let idx = self . view_casters . push (caster) ; unsafe { & * (& raw const self . view_casters [idx]) . cast :: < DatabaseDownCaster < DbView > > () } } # [doc = " Retrieve an downcaster function to `dyn DbView`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the underlying type of `db` is not the same as the database type this downcasts was created for."] pub fn downcaster_for < DbView : ? Sized + Any > (& self) -> & DatabaseDownCaster < DbView > { let view_type_id = TypeId :: of :: < DbView > () ; for (_ , view) in self . view_casters . iter () { if view . target_type_id == view_type_id { return unsafe { & * ((view as * const ViewCaster) . cast :: < DatabaseDownCaster < DbView > > ()) } ; } } panic ! ("No downcaster registered for type `{}` in `Views`" , std :: any :: type_name ::< DbView > () ,) ; } }
    };
}

impl_431!();