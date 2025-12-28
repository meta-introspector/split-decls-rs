macro_rules! LoadingProgress {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum LoadingProgress { Started , Progress (usize) , Finished , }
    };
}

LoadingProgress!()