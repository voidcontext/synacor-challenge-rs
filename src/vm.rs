use core::panic;
use std::{char, convert::TryInto, io, usize};
use Value::{Number, Register};

#[derive(Debug)]
pub struct VM {
    memory: Vec<usize>,
    registers: [usize; 8],
    stack: Vec<usize>,
    pointer: usize,
    input_buffer: Vec<usize>,
}

impl VM {
    pub fn boot() -> VM {
        VM {
            memory: vec![],
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            stack: vec![],
            pointer: 0,
            input_buffer: vec![],
        }
    }

    pub fn load_program(mut self, program: Vec<usize>) -> Self {
        self.memory = program;
        self
    }

    pub fn run(mut self) -> Self {
        log::debug!(
            "running the program loaded into the memory from {}",
            self.pointer
        );
        while self.read_value_at_pointer() != 0 {
            let op_code = self.read_value_at_pointer();
            match op_code {
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
                10 => self.mult(),
                11 => self.r#mod(),
                12 => self.and(),
                13 => self.or(),
                14 => self.not(),
                15 => self.rmem(),
                16 => self.wmem(),
                17 => self.call(),
                18 => self.ret(),
                19 => self.out(),
                20 => self.r#in(),
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
            self.read_value_at_pointer(),
            self.pointer
        )
    }

    fn halt(&mut self) {
        panic!("halt!")
    }

    fn set(&mut self) {
        self.log_opcode("set");

        let reg = self.get_register(self.pointer + 1);

        let value = self.read_value(self.pointer + 2);

        log::debug!("\tregisters before: {:?}", self.registers);
        log::debug!("\tsetting register {} to {}", reg, value);

        self.registers[reg] = value;
        log::debug!("\tregisters after : {:?}", self.registers);

        self.pointer += 3
    }

    fn push(&mut self) {
        self.log_opcode("push");

        let value = self.read_value(self.pointer + 1);
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
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
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
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
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

        let jump_to = self.read_value(self.pointer + 1);
        log::debug!("\tjumping to {}", jump_to);
        self.pointer = jump_to;
    }

    fn jt(&mut self) {
        self.log_opcode("jt");

        if self.read_value(self.pointer + 1) != 0 {
            let jump_to = self.read_value(self.pointer + 2);
            log::debug!("\tjumping to {}", jump_to);
            self.pointer = jump_to
        } else {
            log::debug!("\tnot jumping, moving to next");
            self.pointer += 3;
        }
    }

    fn jf(&mut self) {
        self.log_opcode("jf");

        if self.read_value(self.pointer + 1) == 0 {
            let jump_to = self.read_value(self.pointer + 2);
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
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
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

    fn mult(&mut self) {
        self.log_opcode("mult");

        let register = self.get_register(self.pointer + 1);
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
        let value = modulo(a * b);

        log::debug!(
            "\tsetting register {} to {} * {} = {}",
            register,
            a,
            b,
            value
        );

        self.registers[register] = value;
        self.pointer += 4;
    }

    fn r#mod(&mut self) {
        self.log_opcode("mod");

        let register = self.get_register(self.pointer + 1);
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
        let value = a % b;

        log::debug!(
            "\tsetting register {} to {} % {} = {}",
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
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
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
        let a = self.read_value(self.pointer + 2);
        let b = self.read_value(self.pointer + 3);
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
        let a = self.read_value(self.pointer + 2);
        let value = modulo(!a);

        log::debug!("\tsetting register {} to !{} = {}", register, a, value);

        self.registers[register] = value;
        self.pointer += 3;
    }

    fn rmem(&mut self) {
        self.log_opcode("rmem");

        let register = self.get_register(self.pointer + 1);
        let address = self.read_value(self.pointer + 2);
        let value = self.read_value(address);

        log::debug!(
            "\treading memory from address {} and storing in register {}, value is {}",
            address,
            register,
            value
        );

        self.registers[register] = value;
        self.pointer += 3;
    }

    fn wmem(&mut self) {
        self.log_opcode("wmem");

        let address = self.read_value(self.pointer + 1);
        let value = self.read_value(self.pointer + 2);

        log::debug!(
            "\twriting memory address {} and storing value {}",
            address,
            value
        );

        self.memory[address] = value;
        self.pointer += 3;
    }

    fn call(&mut self) {
        self.log_opcode("call");

        let jump_to = self.read_value(self.pointer + 1);
        let original_next = self.pointer + 2;
        self.stack.push(original_next);

        self.pointer = jump_to;
    }

    fn ret(&mut self) {
        self.log_opcode("ret");
        let jumpt_to = self.stack.pop().unwrap();

        log::debug!("\tjumping to {}", jumpt_to);
        self.pointer = jumpt_to;
    }

    fn out(&mut self) {
        self.log_opcode("out");

        let char_code = self.read_value(self.pointer + 1).try_into().unwrap();
        let char = char::from_u32(char_code).unwrap();
        log::debug!(
            "\tchar {} (code {}) at {}",
            char,
            char_code,
            self.pointer + 1
        );
        //        log::info!("char {} (code {}) at {}:", char, char_code, self.pointer + 1);

        print!("{}", char);

        self.pointer += 2
    }

    fn r#in(&mut self) {
        self.log_opcode("in");

        if self.input_buffer.is_empty() {
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer).unwrap();

            //            println!("read: {}", buffer);

            self.input_buffer = buffer.chars().map(|c| (c as usize)).collect();
            self.input_buffer.reverse();
        }

        //      println!("input buffer: {:?}, pop() into {:?}", self.input_buffer, self.get_value(self.pointer + 1));

        match self.get_value(self.pointer + 1) {
            Number(addr) => self.memory[addr] = self.input_buffer.pop().unwrap(),
            Register(r) => self.registers[r] = self.input_buffer.pop().unwrap(),
        }

        self.pointer += 2;
    }

    fn noop(&mut self) {
        self.log_opcode("noop");
        self.pointer += 1
    }

    fn unimplemented(&mut self) {
        panic!("umimplemented: {}", self.read_value_at_pointer())
    }

    fn read_value_at_pointer(&self) -> usize {
        self.read_value(self.pointer)
    }

    fn read_value(&self, pointer: usize) -> usize {
        match self.get_value(pointer) {
            Number(n) => n,
            Register(r) => self.registers[r],
        }
    }

    fn get_register(&self, pointer: usize) -> usize {
        match self.get_value(pointer) {
            Number(_) => panic!("Expected a register but got number"),
            Register(r) => r,
        }
    }

    fn get_value(&self, pointer: usize) -> Value {
        let value = self.memory[pointer];
        if value < 32768 {
            log::debug!("value is number: {} at {}", value, pointer);
            Number(value)
        } else if (32768..32776).contains(&value) {
            let index: usize = modulo(value);
            Register(index)
        } else {
            panic!("illegal value: {} at {}", value, pointer)
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(usize),
    Register(usize),
}

fn modulo(number: usize) -> usize {
    number % 32768
}
