macro_rules! deps {
    () => {
        SERVICE_STATUS_HANDLE!();
        SERVICE_STATUS!();
        Command!();
    };
}

macro_rules! Service {
    () => {
        deps!();
        # [doc = " A service builder, providing control over what commands the service supports before the service begins to run."] pub struct Service < 'a > { accept : u32 , fallback : Option < Box < dyn FnOnce (& Service) + 'a > > , handle : RwLock < SERVICE_STATUS_HANDLE > , callback : RwLock < Option < Box < dyn FnMut (& Service , Command) + Send + Sync + 'a > > > , status : RwLock < SERVICE_STATUS > , }
    };
}

Service!()