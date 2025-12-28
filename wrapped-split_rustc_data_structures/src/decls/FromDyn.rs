macro_rules! FromDyn {
    () => {
        # [derive (Copy , Clone)] pub struct FromDyn < T > (T) ;
    };
}

FromDyn!()