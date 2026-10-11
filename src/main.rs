use std::io;

const TAPE_LENGTH: usize = 20_000;
const TAPE_INITIAL_POSITION: usize = 0;
const CELL_INITIAL_VALUE: u8 = 0;

#[derive(PartialEq)]
enum BrainFuckError {
    EndOfCodeException,
    NoOpeningBracket,
    NoClosingBracket,
}
struct Tape {
    #[allow(arithmetic_overflow)]
    tape: [u8; TAPE_LENGTH],
    position: usize,
}
impl Tape {
    fn new() -> Self {
        Tape {
            tape: [CELL_INITIAL_VALUE; TAPE_LENGTH],
            position: TAPE_INITIAL_POSITION,
        }
    }
    fn increment(&mut self) {
        self.tape[self.position] += 1;
    }
    fn decrement(&mut self) {
        self.tape[self.position] -= 1;
    }
    fn set(&mut self, value: u8) {
        self.tape[self.position] = value;
    }
    fn move_right(&mut self) {
        match self.position {
            TAPE_LENGTH => (),
            _ => self.position += 1,
        }
    }
    fn move_left(&mut self) {
        match self.position {
            0 => (),
            _ => self.position -= 1,
        }
    }
    fn position(&self) -> usize {
        self.position
    }
    fn check_cell(&self, index: usize) -> Option<u8> {
        self.tape.get(index).copied()
    }
    fn cell(&self) -> u8 {
        self.tape[self.position]
    }
    fn test(&self) {
        println!("");
        for i in 0..20 {
            print!("{} ", self.tape[i])
        }
    }
}


struct Program {
    code: Vec<char>,
    code_position: usize,
    tape: Tape,
}
impl Program {
    fn new(code: String) -> Self {
        Program {
            code: code.chars().collect(),
            code_position: 0,
            tape: Tape::new(),
        }
    }
    fn code_position(&self) -> usize {
        self.code_position
    }
    fn tape_position(&self) -> usize {
        self.tape.position()
    }
    fn is_instruction(c: char) -> bool {
        match c {
            '+' => true,
            '-' => true,
            '>' => true,
            '<' => true,
            '.' => true,
            ',' => true,
            '[' => true,
            ']' => true,
            _ => false,
        }
    }
    fn output(&self) {
        print!("{}", self.tape.cell() as char);
    }
    fn input (&mut self) {
        let stdin = io::stdin(); // We get `Stdin` here.
        let mut buffer: String = String::new();
        loop {
            stdin.read_line(&mut buffer).unwrap();
            if buffer.len() == 1 && let Some(x) = buffer.chars().next() && x.is_ascii() {
                self.tape.set(x as u8);
                return;
            }
        }
    }
    fn loop_open(&mut self) -> Result<(), BrainFuckError> {
        if self.tape.cell() == 0 {
            let mut counter: i32 = 0;
            while self.code_position < self.code.len() && counter >= 0 {
                self.code_position += 1;
                match self.code[self.code_position] {
                    '[' => counter += 1,
                    ']' => counter -= 1,
                    _ => (),
                }
            }
        }
        if self.code_position == self.code.len() {
            return Err(BrainFuckError::NoClosingBracket);
        }
        Ok(())
    }
    fn loop_close(&mut self) -> Result<(), BrainFuckError> {
        if self.tape.cell() != 0 {
            let mut counter: i32 = 0;
            while self.code_position > 0 && counter >= 0 {
                self.code_position -= 1;
                match self.code[self.code_position] {
                    '[' => counter -= 1,
                    ']' => counter += 1,
                    _ => (),
                }
            }
            if self.code_position == 0 {
                return Err(BrainFuckError::NoOpeningBracket);
            }
        }
        Ok(())
    }
    fn next(&mut self) -> Result<(), BrainFuckError> {
        if self.code_position >= self.code.len() {
            return Err(BrainFuckError::EndOfCodeException);
        }
        match self.code[self.code_position] {
            '+' => self.tape.increment(),
            '-' => self.tape.decrement(),
            '<' => self.tape.move_left(),
            '>' => self.tape.move_right(),
            '.' => self.output(),
            ',' => self.input(),
            '[' => self.loop_open()?,
            ']' => self.loop_close()?,
            _ => (),
        }
        self.code_position += 1;
        Ok(())
    }
    fn print(&self) {
        println!("");
        for i in 0..self.code.len() {
            if i == self.code_position { 
                print!("\"{}\"", self.code[i]);
            } else {
                print!("{}", self.code[i]);
            }
        }
    }
    fn test(&self) {
        self.tape.test();
    }
}


fn main() {
    let mut code = String::from("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.");
    let mut program = Program::new(code); 
    loop {
        if program.next() != Ok(()) {
            break;
        }
    }
}
