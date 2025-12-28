macro_rules! deps {
    () => {
        ThreadPoolBuildError!();
        ErrorKind!();
        Registry!();
    };
}

macro_rules! set_global_registry {
    () => {
        deps!();
        # [doc = " Starts the worker threads (if that has not already happened)"] # [doc = " by creating a registry with the given callback."] fn set_global_registry < F > (registry : F) -> Result < & 'static Arc < Registry > , ThreadPoolBuildError > where F : FnOnce () -> Result < Arc < Registry > , ThreadPoolBuildError > , { let mut result = Err (ThreadPoolBuildError :: new (ErrorKind :: GlobalPoolAlreadyInitialized)) ; THE_REGISTRY_SET . call_once (| | { result = registry () . map (| registry : Arc < Registry > | { unsafe { ptr :: addr_of_mut ! (THE_REGISTRY) . write (Some (registry)) ; (* ptr :: addr_of ! (THE_REGISTRY)) . as_ref () . unwrap_unchecked () } }) }) ; result }
    };
}

set_global_registry!()