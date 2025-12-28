macro_rules! UnsafeOpKind {
    () => {
        # [derive (Clone , PartialEq)] enum UnsafeOpKind { CallToUnsafeFunction (Option < DefId >) , UseOfInlineAssembly , InitializingTypeWith , InitializingTypeWithUnsafeField , UseOfMutableStatic , UseOfExternStatic , UseOfUnsafeField , DerefOfRawPointer , AccessToUnionField , MutationOfLayoutConstrainedField , BorrowOfLayoutConstrainedField , CallToFunctionWith { function : DefId , # [doc = " Target features enabled in callee's `#[target_feature]` but missing in"] # [doc = " caller's `#[target_feature]`."] missing : Vec < Symbol > , # [doc = " Target features in `missing` that are enabled at compile time"] # [doc = " (e.g., with `-C target-feature`)."] build_enabled : Vec < Symbol > , } , UnsafeBinderCast , }
    };
}

UnsafeOpKind!();