macro_rules! deps {
    () => {
        EffectIndex!();
        Effect!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl EffectIndex { fn next_in_forward_order (self) -> Self { match self . effect { Effect :: Early => Effect :: Primary . at_index (self . statement_index) , Effect :: Primary => Effect :: Early . at_index (self . statement_index + 1) , } } fn next_in_backward_order (self) -> Self { match self . effect { Effect :: Early => Effect :: Primary . at_index (self . statement_index) , Effect :: Primary => Effect :: Early . at_index (self . statement_index - 1) , } } # [doc = " Returns `true` if the effect at `self` should be applied earlier than the effect at `other`"] # [doc = " in forward order."] fn precedes_in_forward_order (self , other : Self) -> bool { let ord = self . statement_index . cmp (& other . statement_index) . then_with (| | self . effect . cmp (& other . effect)) ; ord == Ordering :: Less } # [doc = " Returns `true` if the effect at `self` should be applied earlier than the effect at `other`"] # [doc = " in backward order."] fn precedes_in_backward_order (self , other : Self) -> bool { let ord = other . statement_index . cmp (& self . statement_index) . then_with (| | self . effect . cmp (& other . effect)) ; ord == Ordering :: Less } }
    };
}

impl_113!()