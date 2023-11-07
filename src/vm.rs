
enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

static DEBUG: bool = true;

impl TryInto<OpCode> for u8 {
    type Error = ();

    fn try_into(self) -> Result<OpCode, Self::Error> {
        match self {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Constant),
            2 => Ok(OpCode::Negate),
            3 => Ok(OpCode::Add),
            4 => Ok(OpCode::Subtract),
            5 => Ok(OpCode::Multiply),
            6 => Ok(OpCode::Divide),
            _ => Err(())
        }
    }
}

pub(crate) struct VM {
    ip: usize,
    stack: [u8; 256],
    stack_pointer: usize,
    memory: Vec<u8>
}

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            ip: 0,
            stack: [0; 256],
            stack_pointer: 0,
            memory: vec![],
        }
    }

    pub(crate) fn interpret(&mut self, bytes: &[u8]) -> u8 {
        while self.ip < bytes.len() {
            // copy program into memory
            for b in bytes {
                self.memory.push(b.clone());
            }

            if DEBUG {
                println!("Stack: {:?}", &self.stack[0..self.stack_pointer]);
            }

            let b = self.memory[self.ip]
                .try_into()
                .expect(&format!("Invalid OpCode {} at instruction {}", bytes[self.ip], self.ip));

            self.ip += 1;
            match b {
                OpCode::Return => {
                    if DEBUG {
                        println!("{}", self.peek());
                    }
                    return self.peek();
                }
                OpCode::Constant => {
                    let constant = bytes[self.ip];
                    self.ip += 1;
                    self.push(constant);
                }
                OpCode::Negate => {
                    let a = self.pop();
                    self.push(a.wrapping_neg());
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_add(b));
                }
                OpCode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_sub(b));
                }
                OpCode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_mul(b));
                }
                OpCode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_div(b));
                }
            }
        }

        return 0;
    }

    fn push(&mut self, byte: u8) {
        if self.stack_pointer + 1 >= self.stack.len() {
            panic!("Stack overflow")
        }
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = byte;
    }

    fn pop(&mut self) -> u8 {
        let b = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
        b
    }

    fn peek(&self) -> u8 {
        self.stack[self.stack_pointer]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let instructions = [
            1, // Constant
            10,
            1, // Constant
            5,
            3, // Add
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        assert_eq!(val, 15)
    }
    #[test]
    fn test_sub() {
        let instructions = [
            1, // Constant
            10,
            1, // Constant
            5,
            4, // Subtract
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        assert_eq!(val, 5)
    }

    #[test]
    fn test_mul() {
        let instructions = [
            1, // Constant
            10,
            1, // Constant
            5,
            5, // Multiply
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        assert_eq!(val, 50)
    }

    #[test]
    fn test_div() {
        let instructions = [
            1, // Constant
            10,
            1, // Constant
            5,
            6, // Divide
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        assert_eq!(val, 2)
    }
}
