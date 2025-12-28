macro_rules! deps {
    () => {
        Action!();
        StreamMock!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Unpin > Drop for StreamMock < T > { fn drop (& mut self) { if std :: thread :: panicking () { return ; } let undropped_count = self . actions . iter () . filter (| action | match action { Action :: Next (_) => true , Action :: Wait (_) => false , }) . count () ; assert ! (undropped_count == 0 , "StreamMock was dropped before all actions were consumed, {undropped_count} actions were not consumed") ; } }
    };
}

impl_24!();