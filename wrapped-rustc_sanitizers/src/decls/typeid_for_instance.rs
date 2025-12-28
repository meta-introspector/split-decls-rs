macro_rules! typeid_for_instance {
    () => {
        # [doc = " Returns a KCFI type metadata identifier for the specified Instance."] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , mut options : TypeIdOptions ,) -> u32 { if matches ! (instance . def , InstanceKind :: ReifyShim (_ , Some (ReifyReason :: FnPtr))) { options . insert (TypeIdOptions :: USE_CONCRETE_SELF) ; } let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_instance (tcx , instance , options) . as_bytes ()) ; hash . finish () as u32 }
    };
}

typeid_for_instance!()