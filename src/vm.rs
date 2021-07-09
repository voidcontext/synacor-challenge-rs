use std::{char, usize};

#[derive(Debug)]
pub struct VM {
    memory: Vec<u16>,
    registers: [u16; 8],
    stack: Vec<u16>,
    pointer: usize,
}

impl VM {
    pub fn boot() -> VM {
        VM {
            memory: vec![],
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            stack: vec![],
            pointer: 0,
        }
    }

    pub fn load_program(mut self, program: Vec<u16>) -> Self {
        self.memory = program;
        self
    }

    pub fn run(mut self) -> Self {
        log::debug!(
            "running the program loaded into the memory from {}",
            self.pointer
        );
        while self.get_value(self.pointer) != 0 {
            match self.get_value(self.pointer) {
                // halt
                0 => {
                    log::debug!("opcode 0 (halt) at {}", self.pointer);
                    self.pointer += 1
                }
                //set
                1 => {
                    log::debug!("opcode 1 (set) at {}", self.pointer);
                    log::debug!("\tregisters: {:?}", self.registers);
                    let reg: usize = self.get_register(self.pointer + 1);
                    let value = self.get_value(self.pointer + 2);
                    self.registers[reg] = value;
                    log::debug!("\tsetting register {} to {}", reg, value);
                    log::debug!("\tregisters: {:?}", self.registers);

                    self.pointer += 3
                }
                // jmp
                6 => {
                    let jump_to = self.get_value(self.pointer + 1);
                    log::debug!("opcode 6 (jmp) at {}, jumping to {}", self.pointer, jump_to);
                    self.pointer = jump_to.into();
                }
                // jt (jump if nonzero)
                7 => {
                    log::debug!(
                        "opcode 7 (jt) at {} value is {}",
                        self.pointer,
                        self.get_value(self.pointer + 1)
                    );
                    if self.get_value(self.pointer + 1) != 0 {
                        let jump_to = self.get_value(self.pointer + 2);
                        log::debug!("\tjumping to {}", jump_to);
                        self.pointer = jump_to.into()
                    } else {
                        log::debug!("\tnot jumping, moving to next");
                        self.pointer += 3;
                    }
                }
                // jf (jump if zero)
                8 => {
                    log::debug!(
                        "opcode 8 (jf) at {} value is {}",
                        self.pointer,
                        self.get_value(self.pointer + 1)
                    );
                    if self.get_value(self.pointer + 1) == 0 {
                        let jump_to = self.get_value(self.pointer + 2).into();
                        log::debug!("\tjumping to {}", jump_to);
                        self.pointer = jump_to;
                    } else {
                        log::debug!("\tnot jumping, moving to next");
                        self.pointer += 3;
                    }
                }
                // out
                19 => {
                    log::debug!("opcode 19 (out) at {}", self.pointer);
                    let char = char::from_u32(self.get_value(self.pointer + 1).into()).unwrap();
                    log::debug!("char {} at {}", char, self.pointer + 1);
                    print!("{}", char);

                    self.pointer += 2
                }
                // noop
                21 => {
                    log::debug!("opcode 21 (noop) at {}", self.pointer);
                    self.pointer += 1
                }
                code => panic!("Unimplemented op code: {} at {}", code, self.pointer),
            }
        }
        self
    }

    fn get_register(&self, pointer: usize) -> usize {
        let value = self.memory[pointer];

        if (32768..32776).contains(&value) {
            let index: usize = (value % 32768).into();

            log::debug!(
                "value -> registers[{} mod 32768 = {}] = {} at {}",
                value,
                index,
                self.registers[index],
                pointer
            );

            index
        } else {
            panic!("illegal register: {} at {}", value, pointer);
        }
    }

    fn get_value(&self, pointer: usize) -> u16 {
        let value = self.memory[pointer];
        if value < 32768 {
            log::debug!("value is number: {} at {}", value, pointer);
            value
        } else if (32768..32776).contains(&value) {
            let index: usize = (value % 32768).into();

            log::debug!(
                "value from register: registers[{} mod 32768 = {}] = {} at {}",
                value,
                index,
                self.registers[index],
                pointer
            );
            self.registers[index]
        } else {
            panic!("illegal value: {} at {}", value, pointer)
        }
    }
}

// union NumberOrOperation {
//     number: Number,
//     operation: Operation
// }

// pub struct Number {
//     pub value: u16
// }

// pub enum Operation {
//     Halt,
//     Out(u16),
//     Noop,
// }

// pub impl Operation {
//     pub fn from(code: u16) -> NumberOrOperation {
//     }
// }
