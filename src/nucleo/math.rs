use crate::qud::Qud;

pub fn qadd(a: Qud, b: Qud) -> Qud {
    a + b
}

pub fn qmul(a: Qud, b: Qud) -> Qud {
    a * b
}

pub fn qmax(a: Qud, b: Qud) -> Qud {
    a.qmax(b)
}

pub fn qmin(a: Qud, b: Qud) -> Qud {
    a.qmin(b)
}

pub fn collapse(q: Qud) -> Qud {
    match q {
        Qud::Super => Qud::One,
        Qud::Error => Qud::Zero,
        other => other,
    }
}
