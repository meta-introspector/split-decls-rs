macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < S > Stream for StreamNotifyClose < S > where S : Stream , { type Item = Option < S :: Item > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . as_mut () . project () . inner . as_pin_mut () . map (| stream | S :: poll_next (stream , cx)) { Some (Poll :: Ready (Some (item))) => Poll :: Ready (Some (Some (item))) , Some (Poll :: Ready (None)) => { self . project () . inner . set (None) ; Poll :: Ready (Some (None)) } Some (Poll :: Pending) => Poll :: Pending , None => Poll :: Ready (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if let Some (inner) = & self . inner { let (l , u) = inner . size_hint () ; (l . saturating_add (1) , u . and_then (| u | u . checked_add (1))) } else { (0 , Some (0)) } } }
    };
}

impl_79!();