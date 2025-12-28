macro_rules! deps {
    () => {
        Reason!();
        Answer!();
        Condition!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < R , T > Answer < R , T > { fn and (self , rhs : Answer < R , T >) -> Answer < R , T > { let lhs = self ; match (lhs , rhs) { (Answer :: No (Reason :: DstIsBitIncompatible) , Answer :: No (reason)) | (Answer :: No (reason) , Answer :: No (_)) | (Answer :: No (reason) , _) | (_ , Answer :: No (reason)) => Answer :: No (reason) , | (Answer :: Yes , other) | (other , Answer :: Yes) => other , (Answer :: If (Condition :: IfAll (mut lhs)) , Answer :: If (Condition :: IfAll (ref mut rhs))) => { lhs . append (rhs) ; Answer :: If (Condition :: IfAll (lhs)) } (Answer :: If (cond) , Answer :: If (Condition :: IfAll (mut conds))) | (Answer :: If (Condition :: IfAll (mut conds)) , Answer :: If (cond)) => { conds . push (cond) ; Answer :: If (Condition :: IfAll (conds)) } (Answer :: If (lhs) , Answer :: If (rhs)) => Answer :: If (Condition :: IfAll (vec ! [lhs , rhs])) , } } fn or (self , rhs : Answer < R , T >) -> Answer < R , T > { let lhs = self ; match (lhs , rhs) { (Answer :: No (Reason :: DstIsBitIncompatible) , Answer :: No (reason)) | (Answer :: No (reason) , Answer :: No (_)) => Answer :: No (reason) , (Answer :: No (_) , other) | (other , Answer :: No (_)) => other . or (Answer :: Yes) , (Answer :: Yes , other) | (other , Answer :: Yes) => other , (Answer :: If (Condition :: IfAny (mut lhs)) , Answer :: If (Condition :: IfAny (ref mut rhs))) => { lhs . append (rhs) ; Answer :: If (Condition :: IfAny (lhs)) } (Answer :: If (cond) , Answer :: If (Condition :: IfAny (mut conds))) | (Answer :: If (Condition :: IfAny (mut conds)) , Answer :: If (cond)) => { conds . push (cond) ; Answer :: If (Condition :: IfAny (conds)) } (Answer :: If (lhs) , Answer :: If (rhs)) => Answer :: If (Condition :: IfAny (vec ! [lhs , rhs])) , } } }
    };
}

impl_65!()