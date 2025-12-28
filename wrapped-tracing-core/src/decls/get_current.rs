macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! get_current {
    () => {
        deps!();
        # [doc = " Executes a closure with a reference to the current [dispatcher]."] # [doc = ""] # [doc = " [dispatcher]: super::dispatcher::Dispatch"] # [cfg (not (feature = "std"))] # [doc (hidden)] pub fn get_current < T > (f : impl FnOnce (& Dispatch) -> T) -> Option < T > { Some (f (get_global ())) }
    };
}

get_current!();