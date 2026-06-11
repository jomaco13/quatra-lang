use crate::vm::stack::QudStack;
use crate::qud::Qud;

pub fn qadd_simd(_stack: &mut QudStack) {
    let b = _stack.pop();
    let a = _stack.pop();
    _stack.push(a + b);
}

pub fn qmul_simd(_stack: &mut QudStack) {
    let b = _stack.pop();
    let a = _stack.pop();
    _stack.push(a * b);
}

pub fn tensor_collapse(_stack: &mut QudStack) {
    let val = _stack.pop();
    _stack.push(match val {
        Qud::Super => Qud::One,
        Qud::Error => Qud::Zero,
        _ => val,
    });
}
