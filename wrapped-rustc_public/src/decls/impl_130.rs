macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Rvalue < 'tcx > { type T = crate :: mir :: Rvalue ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: Rvalue :: * ; match self { Use (op) => crate :: mir :: Rvalue :: Use (op . stable (tables , cx)) , Repeat (op , len) => { let len = len . stable (tables , cx) ; crate :: mir :: Rvalue :: Repeat (op . stable (tables , cx) , len) } Ref (region , kind , place) => crate :: mir :: Rvalue :: Ref (region . stable (tables , cx) , kind . stable (tables , cx) , place . stable (tables , cx) ,) , ThreadLocalRef (def_id) => { crate :: mir :: Rvalue :: ThreadLocalRef (tables . crate_item (* def_id)) } RawPtr (mutability , place) => crate :: mir :: Rvalue :: AddressOf (mutability . stable (tables , cx) , place . stable (tables , cx) ,) , Len (place) => crate :: mir :: Rvalue :: Len (place . stable (tables , cx)) , Cast (cast_kind , op , ty) => crate :: mir :: Rvalue :: Cast (cast_kind . stable (tables , cx) , op . stable (tables , cx) , ty . stable (tables , cx) ,) , BinaryOp (bin_op , ops) => { if let Some (bin_op) = bin_op . overflowing_to_wrapping () { crate :: mir :: Rvalue :: CheckedBinaryOp (bin_op . stable (tables , cx) , ops . 0 . stable (tables , cx) , ops . 1 . stable (tables , cx) ,) } else { crate :: mir :: Rvalue :: BinaryOp (bin_op . stable (tables , cx) , ops . 0 . stable (tables , cx) , ops . 1 . stable (tables , cx) ,) } } NullaryOp (null_op , ty) => { crate :: mir :: Rvalue :: NullaryOp (null_op . stable (tables , cx) , ty . stable (tables , cx)) } UnaryOp (un_op , op) => { crate :: mir :: Rvalue :: UnaryOp (un_op . stable (tables , cx) , op . stable (tables , cx)) } Discriminant (place) => crate :: mir :: Rvalue :: Discriminant (place . stable (tables , cx)) , Aggregate (agg_kind , operands) => { let operands = operands . iter () . map (| op | op . stable (tables , cx)) . collect () ; crate :: mir :: Rvalue :: Aggregate (agg_kind . stable (tables , cx) , operands) } ShallowInitBox (op , ty) => { crate :: mir :: Rvalue :: ShallowInitBox (op . stable (tables , cx) , ty . stable (tables , cx)) } CopyForDeref (place) => crate :: mir :: Rvalue :: CopyForDeref (place . stable (tables , cx)) , WrapUnsafeBinder (..) => todo ! ("FIXME(unsafe_binders):") , } } }
    };
}

impl_130!();