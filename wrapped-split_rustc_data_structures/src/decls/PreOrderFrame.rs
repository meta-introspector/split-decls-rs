macro_rules! PreOrderFrame {
    () => {
        struct PreOrderFrame < Iter > { pre_order_idx : PreorderIndex , iter : Iter , }
    };
}

PreOrderFrame!()