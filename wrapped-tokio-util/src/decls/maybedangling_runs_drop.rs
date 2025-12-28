macro_rules! deps {
    () => {
        MaybeDangling!();
    };
}

macro_rules! maybedangling_runs_drop {
    () => {
        deps!();
        # [test] fn maybedangling_runs_drop () { struct SetOnDrop < 'a > (& 'a mut bool) ; impl Drop for SetOnDrop < '_ > { fn drop (& mut self) { * self . 0 = true ; } } let mut success = false ; drop (MaybeDangling :: new (SetOnDrop (& mut success))) ; assert ! (success) ; }
    };
}

maybedangling_runs_drop!();