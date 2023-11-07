use crate::vm::VM;

mod vm;

fn main() {
    let instruction = [
        1,
        10,
        1,
        5,
        3,
        0,
    ];

    let mut vm = VM::new();
    vm.interpret(&instruction);
}
