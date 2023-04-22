use std::io;

mod helper;

#[derive(Debug)]
struct Program {
    input: String,
    accumulator: i64,
    instruction_pointer: u32,
    bool_flag: bool,
    counter: u32,
    stack: Vec<i64>,
}

impl Program {
    pub fn execute(&mut self) {
        let command = match self
            .input
            .chars()
            .nth(self.instruction_pointer.try_into().unwrap())
        {
            Some(c) => c,
            _ => '\0',
        };

        let next_command = match self
            .input
            .chars()
            .nth((self.instruction_pointer + 1).try_into().unwrap())
        {
            Some(c) => match c.to_digit(10) {
                Some(d) => d,
                _ => 0,
            },
            _ => 0,
        };

        match command {
            'i' => self.accumulator += 1,
            'I' =>  {
                for x in self.stack.iter_mut() {
                    *x += 1;
                }
            }
            'd' => self.accumulator -= 1,
            'D' => {
                for x in self.stack.iter_mut() {
                    *x -= 1;
                }
            }
            's' => self.accumulator *= self.accumulator,
            'S' => todo!(),
            'o' => {
                print!("{}", self.accumulator)
            }
            'c' => match char::from_u32(self.accumulator.try_into().unwrap()) {
                Some(i) => print!("{}", i),
                _ => print!("{}", self.accumulator),
            },
            'C' => {
                self.counter = self.accumulator.try_into().unwrap();
            }
            'n' => {
                self.accumulator = helper::next_prime(self.accumulator.try_into().unwrap()) as i64
            }
            'p' => {
                self.accumulator =
                    helper::previous_prime(self.accumulator.try_into().unwrap()) as i64
            }
            'v' => self.stack.push(self.accumulator),
            '^' => {
                self.accumulator = {
                    match self.stack.pop() {
                        Some(i) => i,
                        _ => 0,
                    }
                }
            }

            'r' => self.accumulator = 0,
            'R' => self.accumulator = 1,
            'f' => self.accumulator = 69,
            'b' => match self.bool_flag {
                true => {}
                false => {
                    self.instruction_pointer += next_command;
                }
            },
            'T' => self.bool_flag = true,
            'F' => self.bool_flag = false,
            'L' => {
                if self.counter > 0 {
                    self.instruction_pointer -= next_command + 1;
                    self.counter -= 1;
                } else {
                    self.instruction_pointer += 1
                }
            }
            'l' => {
                if self.counter > 0 {
                    self.instruction_pointer -= next_command + 1;
                } else {
                    self.instruction_pointer += 1;
                }
            }
            'w' => todo!(),
            'm' => todo!(),
            'e' => {
                self.bool_flag = match self.accumulator % 2 {
                    0 => true,
                    1 => false,
                    _ => false,
                }
            }
            'k' => {
                self.instruction_pointer += next_command;
            }
            'K' => {
                self.instruction_pointer -= next_command;
                return;
            }
            '.' => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let trimmed = input.trim();

                let x = if let Ok(num) = trimmed.parse::<i64>() {
                    num
                } else if let Some(c) = trimmed.chars().next() {
                    c as i64
                } else {
                    return;
                };

                self.accumulator = x;
            }
            ' ' => print!(" "),
            '_' => println!(),
            _ => {}
        }
        self.instruction_pointer += 1;
        self.instruction_pointer
            .min(self.input.chars().count() as u32)
            .max(0);
    }

    pub fn parse(&mut self) {
        loop {
            if self.instruction_pointer >= self.input.chars().count().try_into().unwrap() {
                return;
            }

            self.execute();
            self.debug_info();
        }
    }

    pub fn debug_info(&self) {
        println!("{:?}", self)
    }

    pub fn new(input_string: String) -> Self {
        Self {
            input: input_string,
            accumulator: 0,
            instruction_pointer: 0,
            bool_flag: false,
            counter: 0,
            stack: Vec::new(),
        }
    }
}

fn main() {
    let mut p = Program::new(".v.vI_".to_string());
    p.debug_info();
    p.parse();
}
