
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
    stack: Vec<u8>,
}

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            ip: 0,
            stack: vec![],
        }
    }

    pub(crate) fn interpret(&mut self, bytes: &[u8]) -> &u8 {
        while self.ip < bytes.len() {
            if DEBUG {
                println!("Stack: {:?}", self.stack);
            }
            let b = bytes[self.ip].try_into().expect(&format!("Invalid OpCode {} at instruction {}", bytes[self.ip], self.ip));
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
                    self.stack.push(constant);
                }
                OpCode::Negate => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_neg());
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_add(b));
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_sub(b));
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_mul(b));
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_div(b));
                }
            }
        }

        return &0;
    }

    fn peek(&self) -> &u8 {
        self.stack.last().unwrap()
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
        assert_eq!(val, &15)
    }
}
