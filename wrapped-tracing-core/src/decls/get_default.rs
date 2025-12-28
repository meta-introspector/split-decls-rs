macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! get_default {
    () => {
        deps!();
        # [doc = " Executes a closure with a reference to the current [dispatcher]."] # [doc = ""] # [doc = " [dispatcher]: super::dispatcher::Dispatch"] # [cfg (not (feature = "std"))] pub fn get_default < T , F > (mut f : F) -> T where F : FnMut (& Dispatch) -> T , { f (& get_global ()) }
    };
}

get_default!();