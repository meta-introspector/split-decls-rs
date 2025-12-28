macro_rules! deps {
    () => {
        MaybeDangling!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < F : Future > Future for MaybeDangling < F > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let fut = unsafe { self . map_unchecked_mut (| this | this . 0 . assume_init_mut ()) } ; fut . poll (cx) } }
    };
}

impl_111!()