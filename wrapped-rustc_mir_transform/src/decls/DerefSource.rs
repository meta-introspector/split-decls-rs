macro_rules! DerefSource {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq)] enum DerefSource { # [doc = " `fn shim(&self) { inner(*self )}`."] ImmRef , # [doc = " `fn shim(&mut self) { inner(*self )}`."] MutRef , # [doc = " `fn shim(*mut self) { inner(*self )}`."] MutPtr , }
    };
}

DerefSource!()