macro_rules! deps {
    () => {
        JoinMap!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V , S > fmt :: Debug for JoinMap < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct KeySet < 'a , K : fmt :: Debug > (& 'a HashTable < (K , AbortHandle) >) ; impl < K : fmt :: Debug > fmt :: Debug for KeySet < '_ , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . 0 . iter () . map (| (key , abort) | (key , abort . id ()))) . finish () } } f . debug_struct ("JoinMap") . field ("tasks" , & KeySet (& self . tasks_by_key)) . finish () } }
    };
}

impl_21!()