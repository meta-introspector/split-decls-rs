macro_rules! impl_84 {
    () => {
        impl Default for gz_header { fn default () -> Self { Self { text : 0 , time : 0 , xflags : 0 , os : 0 , extra : core :: ptr :: null_mut () , extra_len : 0 , extra_max : 0 , name : core :: ptr :: null_mut () , name_max : 0 , comment : core :: ptr :: null_mut () , comm_max : 0 , hcrc : 0 , done : 0 , } } }
    };
}

impl_84!()