use core::panic;
use std::{char, convert::TryInto, usize};

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
        while self.get_value_at_pointer() != 0 {
            match self.get_value_at_pointer() {
                0 => self.halt(),
                1 => self.set(),
                2 => self.push(),
                3 => self.pop(),
                4 => self.eq(),
                5 => self.gt(),
                6 => self.jmp(),
                7 => self.jt(),
                8 => self.jf(),
                9 => self.add(),
                12 => self.and(),
                13 => self.or(),
                14 => self.not(),
                17 => self.call(),
                19 => self.out(),
                21 => self.noop(),
                _ => self.unimplemented(),
            }
        }
        self
    }

    fn log_opcode(&self, name: &str) {
        log::debug!(
            "found instruction {} (opcode {}) at {}",
            name,
            self.get_value_at_pointer(),
            self.pointer
        )
    }

    fn halt(&mut self) {
        panic!("halt!")
    }

    fn set(&mut self) {
        self.log_opcode("set");

        let reg: usize = self.get_register(self.pointer + 1);
        let value = self.get_value(self.pointer + 2);

        log::debug!("\tregisters before: {:?}", self.registers);
        log::debug!("\tsetting register {} to {}", reg, value);

        self.registers[reg] = value;
        log::debug!("\tregisters after : {:?}", self.registers);

        self.pointer += 3
    }

    fn push(&mut self) {
        self.log_opcode("push");

        let value = self.get_value(self.pointer + 1);
        self.stack.push(value);

        self.pointer += 2;
    }

    fn pop(&mut self) {
        self.log_opcode("pop");

        let register = self.get_register(self.pointer + 1);

        let value = self.stack.pop().unwrap();
        self.registers[register] = value;

        self.pointer += 2;
    }

    fn eq(&mut self) {
        self.log_opcode("eq");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let b = self.get_value(self.pointer + 3);
        let value = if a == b { 1 } else { 0 };

        log::debug!(
            "\tsetting register {} to {} == {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn gt(&mut self) {
        self.log_opcode("gt");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let b = self.get_value(self.pointer + 3);
        let value = if a > b { 1 } else { 0 };

        log::debug!(
            "\tsetting register {} to {} < {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn jmp(&mut self) {
        self.log_opcode("jmp");

        let jump_to = self.get_value(self.pointer + 1);
        log::debug!("\tjumping to {}", jump_to);
        self.pointer = jump_to.into();
    }

    fn jt(&mut self) {
        self.log_opcode("jt");

        if self.get_value(self.pointer + 1) != 0 {
            let jump_to = self.get_value(self.pointer + 2);
            log::debug!("\tjumping to {}", jump_to);
            self.pointer = jump_to.into()
        } else {
            log::debug!("\tnot jumping, moving to next");
            self.pointer += 3;
        }
    }

    fn jf(&mut self) {
        self.log_opcode("jf");

        if self.get_value(self.pointer + 1) == 0 {
            let jump_to = self.get_value(self.pointer + 2).into();
            log::debug!("\tjumping to {}", jump_to);
            self.pointer = jump_to;
        } else {
            log::debug!("\tnot jumping, moving to next");
            self.pointer += 3;
        }
    }

    fn add(&mut self) {
        self.log_opcode("add");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let b = self.get_value(self.pointer + 3);
        let value = modulo(a + b);

        log::debug!(
            "\tsetting register {} to {} + {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn and(&mut self) {
        self.log_opcode("and");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let b = self.get_value(self.pointer + 3);
        let value = modulo(a & b);

        log::debug!(
            "\tsetting register {} to {} & {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn or(&mut self) {
        self.log_opcode("or");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let b = self.get_value(self.pointer + 3);
        let value = modulo(a | b);

        log::debug!(
            "\tsetting register {} to {} | {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn not(&mut self) {
        self.log_opcode("not");

        let register = self.get_register(self.pointer + 1);
        let a = self.get_value(self.pointer + 2);
        let value = modulo(!a);

        log::debug!("\tsetting register {} to !{} = {}", register, a, value);

        self.registers[register] = value;
        self.pointer += 3;
    }

    fn call(&mut self) {
        self.log_opcode("call");

        let jump_to = self.get_value(self.pointer + 1);
        let original_next: u16 = (self.pointer + 2).try_into().unwrap();
        self.stack.push(original_next);

        self.pointer = jump_to.into();
    }

    fn out(&mut self) {
        self.log_opcode("out");

        let char = char::from_u32(self.get_value(self.pointer + 1).into()).unwrap();
        log::debug!("\tchar {} at {}", char, self.pointer + 1);
        print!("{}", char);

        self.pointer += 2
    }

    fn noop(&mut self) {
        self.log_opcode("noop");
        self.pointer += 1
    }

    fn unimplemented(&mut self) {
        panic!("umimplemented: {}", self.get_value_at_pointer())
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

    fn get_value_at_pointer(&self) -> u16 {
        self.get_value(self.pointer)
    }

    fn get_value(&self, pointer: usize) -> u16 {
        let value = self.memory[pointer];
        if value < 32768 {
            log::debug!("value is number: {} at {}", value, pointer);
            value
        } else if (32768..32776).contains(&value) {
            let index: usize = modulo(value).into();

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

fn modulo(number: u16) -> u16 {
    number % 32768
}
