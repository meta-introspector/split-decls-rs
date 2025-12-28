macro_rules! deps {
    () => {
        StreamMap!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < K , V > Stream for StreamMap < K , V > where K : Clone + Unpin , V : Stream + Unpin , { type Item = (K , V :: Item) ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { if let Some ((idx , val)) = ready ! (self . poll_next_entry (cx)) { let key = self . entries [idx] . 0 . clone () ; Poll :: Ready (Some ((key , val))) } else { Poll :: Ready (None) } } fn size_hint (& self) -> (usize , Option < usize >) { let mut ret = (0 , Some (0)) ; for (_ , stream) in & self . entries { let hint = stream . size_hint () ; ret . 0 += hint . 0 ; match (ret . 1 , hint . 1) { (Some (a) , Some (b)) => ret . 1 = Some (a + b) , (Some (_) , None) => ret . 1 = None , _ => { } } } ret } }
    };
}

impl_72!();