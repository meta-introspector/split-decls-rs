macro_rules! deps {
    () => {
        ErasedDatabaseDownCasterSig!();
    };
}

macro_rules! ViewCaster {
    () => {
        deps!();
        # [derive (Copy , Clone)] struct ViewCaster { # [doc = " The id of the target type `dyn DbView` that we can cast to."] target_type_id : TypeId , # [doc = " The name of the target type `dyn DbView` that we can cast to."] type_name : & 'static str , # [doc = " Type-erased function pointer that downcasts to `dyn DbView`."] cast : ErasedDatabaseDownCasterSig , }
    };
}

ViewCaster!();