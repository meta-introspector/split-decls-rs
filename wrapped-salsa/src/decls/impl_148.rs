macro_rules! deps {
    () => {
        Ingredient!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl dyn Ingredient { # [doc = " Equivalent to the `downcast` method on `Any`."] # [doc = ""] # [doc = " Because we do not have dyn-downcasting support, we need this workaround."] pub fn assert_type < T : Any > (& self) -> & T { assert_eq ! (self . type_id () , TypeId :: of ::< T > () , "ingredient `{self:?}` is not of type `{}`" , std :: any :: type_name ::< T > ()) ; unsafe { transmute_data_ptr (self) } } # [doc = " Equivalent to the `downcast` methods on `Any`."] # [doc = ""] # [doc = " Because we do not have dyn-downcasting support, we need this workaround."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The contained value must be of type `T`."] pub unsafe fn assert_type_unchecked < T : Any > (& self) -> & T { debug_assert_eq ! (self . type_id () , TypeId :: of ::< T > () , "ingredient `{self:?}` is not of type `{}`" , std :: any :: type_name ::< T > ()) ; unsafe { transmute_data_ptr (self) } } # [doc = " Equivalent to the `downcast` method on `Any`."] # [doc = ""] # [doc = " Because we do not have dyn-downcasting support, we need this workaround."] pub fn assert_type_mut < T : Any > (& mut self) -> & mut T { assert_eq ! (Any :: type_id (self) , TypeId :: of ::< T > () , "ingredient `{self:?}` is not of type `{}`" , std :: any :: type_name ::< T > ()) ; unsafe { transmute_data_mut_ptr (self) } } }
    };
}

impl_148!()