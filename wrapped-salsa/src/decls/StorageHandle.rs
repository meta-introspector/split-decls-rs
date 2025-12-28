macro_rules! deps {
    () => {
        Zalsa!();
        CoordinateDrop!();
    };
}

macro_rules! StorageHandle {
    () => {
        deps!();
        # [doc = " A handle to non-local database state."] pub struct StorageHandle < Db > { # [doc = " Reference to the database."] zalsa_impl : Arc < Zalsa > , # [doc = " Coordination data for cancellation of other handles when `zalsa_mut` is called."] # [doc = " This could be stored in Zalsa but it makes things marginally cleaner to keep it separate."] coordinate : CoordinateDrop , # [doc = " We store references to `Db`"] phantom : PhantomData < fn () -> Db > , }
    };
}

StorageHandle!();