macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < S , R > tower_service :: Service < R > for Service < S > where S : tower_service :: Service < R > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let _enter = self . span . enter () ; self . inner . poll_ready (cx) } fn call (& mut self , request : R) -> Self :: Future { let _enter = self . span . enter () ; self . inner . call (request) } }
    };
}

impl_11!()