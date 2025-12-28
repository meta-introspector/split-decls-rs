macro_rules! deps {
    () => {
        Tree!();
        Scope!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < T : Send > Tree < T > { fn iter (& self) -> vec :: IntoIter < & T > { once (& self . value) . chain (self . children . iter () . flat_map (Tree :: iter)) . collect :: < Vec < _ > > () . into_iter () } fn update < OP > (& mut self , op : OP) where OP : Fn (& mut T) + Sync , T : Send , { scope (| s | self . update_in_scope (& op , s)) ; } fn update_in_scope < 'scope , OP > (& 'scope mut self , op : & 'scope OP , scope : & Scope < 'scope >) where OP : Fn (& mut T) + Sync , { let Tree { ref mut value , ref mut children } = * self ; scope . spawn (move | scope | { for child in children { scope . spawn (move | scope | child . update_in_scope (op , scope)) ; } }) ; op (value) ; } }
    };
}

impl_139!();