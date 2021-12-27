use std::str::FromStr;
use crate::days::Day;
use crate::days::day24::Value::{Literal, Variable};
use crate::util::number::parse_isize;

pub const DAY24: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    // There might be a way to get to this code-wise, e.g. by inspecting values during execution.
    // But reverse engineering the code was simpler, and showed the serial had pairs that would
    // cancel:
    // N14 = N1 - 6
    // N13 = N2 - 2
    // N12 = N7 - 1
    // N11 = N8 + 6
    // N10 = N9 + 8
    // N6  = N5 + 1
    // N4  = N3 + 7
    // Maximizing numbers, yields us:
    // N14 = 3, N1 = 9
    // N13 = 7, N2 = 9
    // N12 = 8, N7 = 9
    // N11 = 9, N8 = 3,
    // N10 = 9, N9 = 1,
    // N6  = 9, N5 = 8,
    // N4  = 9, N3 = 2
    // 99298993199873
    let mut alu = ALU::default();
    alu.input = vec![9,9,2,9,8,9,9,3,1,9,9,8,7,3];
    alu.run(input).unwrap();
    if alu.z == 0 {
        println!("Puzzle 1 answer should be: 99298993199873")
    } else {
        println!("Puzzle 1 answer is still to be found...");
    }
}
fn puzzle2(input: &String) {
    // N14 = N1 - 6
    // N13 = N2 - 2
    // N12 = N7 - 1
    // N11 = N8 + 6
    // N10 = N9 + 8
    // N6  = N5 + 1
    // N4  = N3 + 7
    // Minimizing numbers, yields us:
    // N14 = 1, N1 = 7
    // N13 = 1, N2 = 3
    // N12 = 1, N7 = 2
    // N11 = 7, N8 = 1,
    // N10 = 9, N9 = 1,
    // N6  = 2, N5 = 1,
    // N4  = 8, N3 = 1
    // 73181221197111
    let mut alu = ALU::default();
    alu.input = vec![7,3,1,8,1,2,2,1,1,9,7,1,1,1];
    alu.run(input).unwrap();
    if alu.z == 0 {
        println!("Puzzle 2 answer should be: 73181221197111")
    } else {
        println!("Puzzle 2 answer is still to be found...");
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Value {
    Variable(char),
    Literal(isize)
}

impl FromStr for Value {
    type Err = ALUError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let char = s.chars().nth(0);
            match char {
                Some('w') | Some('x') | Some('y') | Some('z') => return Ok(Variable(char.unwrap())),
                _ => {}
            }
        }
        
        Ok(Literal(parse_isize(s).map_err(|e| ALUError::InvalidArg(e))?))
    }
}

impl Value {
    fn get(&self, alu: &ALU) -> Result<isize, ALUError> {
        match self {
            Literal(v) => Ok(v.clone()),
            Variable('w') => Ok(alu.w),
            Variable('x') => Ok(alu.x),
            Variable('y') => Ok(alu.y),
            Variable('z') => Ok(alu.z),
            Variable(v) => Err(ALUError::InvalidVariable(v.clone()))
        }
    }

    fn get_mut<'a>(&self, alu: &'a mut ALU) -> Result<&'a mut isize, ALUError> {
        match self {
            Variable('w') => Ok(&mut alu.w),
            Variable('x') => Ok(&mut alu.x),
            Variable('y') => Ok(&mut alu.y),
            Variable('z') => Ok(&mut alu.z),
            Variable(v) => Err(ALUError::InvalidVariable(v.clone())),
            Literal(_) => Err(ALUError::InvalidLiteral)
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct ALU {
    input: Vec<isize>,
    w: isize,
    x: isize,
    y: isize,
    z: isize
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum ALUError {
    InvalidCommand(String),
    InvalidArg(String),
    InvalidVariable(char),
    InvalidLiteral,
    NoInput,
    DivByZero,
    NegMod,
    ModByZero,
    ModByNeg
}

impl ALU {
    fn run(&mut self, program: &str) -> Result<(), ALUError> {
        for cmd in program.lines().filter(|l| !l.is_empty()) {
            self.cmd(cmd)?;
        }
        
        Ok(())
    }
    
    fn cmd(&mut self, command: &str) -> Result<(), ALUError> {
        let parts: Vec<_> = command.split(" ").collect();
        
        match parts[0] {
            "inp" => self.inp(parts[1].parse()?),
            "add" => self.add(parts[1].parse()?, parts[2].parse()?),
            "mul" => self.mul(parts[1].parse()?, parts[2].parse()?),
            "div" => self.div(parts[1].parse()?, parts[2].parse()?),
            "mod" => self.modulo(parts[1].parse()?, parts[2].parse()?),
            "eql" => self.eql(parts[1].parse()?, parts[2].parse()?),
            inv => Err(ALUError::InvalidCommand(format!("Invalid command {}", inv)))
        }
    }
    
    fn inp(&mut self, var: Value) -> Result<(), ALUError> {
        if self.input.len() == 0 {
            return Err(ALUError::NoInput);
        }
        
        let input = self.input.remove(0);
        *var.get_mut(self)? = input;
        Ok(())
    }
    
    fn add(&mut self, a: Value, b: Value) -> Result<(), ALUError> {
        *a.get_mut(self)? += b.get(self)?;
        Ok(())
    }
    
    fn mul(&mut self, a: Value, b: Value) -> Result<(), ALUError> {
        *a.get_mut(self)? *= b.get(self)?;
        Ok(())
    }
    
    fn div(&mut self, a: Value, b: Value) -> Result<(), ALUError> {
        let rhs = b.get(self)?;
        if rhs == 0 { return Err(ALUError::DivByZero) }
        *a.get_mut(self)? /= rhs;
        Ok(())
    }
    
    fn modulo(&mut self, a: Value, b: Value) -> Result<(), ALUError> {
        let rhs = b.get(self)?;
        if rhs == 0 { return Err(ALUError::ModByZero) }
        if rhs < 0 { return Err(ALUError::ModByNeg) }
        let lhs = a.get_mut(self)?;
        if *lhs < 0 { return Err(ALUError::NegMod) }
        *lhs %= rhs;
        Ok(())
    }
    
    fn eql(&mut self, a: Value, b: Value) -> Result<(), ALUError> {
        let rhs = b.get(self)?;
        let lhs = a.get_mut(self)?;
        *lhs = if rhs.eq(lhs) { 1 } else { 0 }; 
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day24::ALU;

    #[test]
    fn test_alu() {
        let mut alu = ALU::default();
        
        alu.input.push(42);
        assert_eq!(alu.cmd("inp x"), Ok(()));
        assert_eq!(alu.x, 42);
        assert_eq!(alu.input, Vec::<isize>::new());
        
        assert_eq!(alu.cmd("mul x -1"), Ok(()));
        assert_eq!(alu.x, -42);
        
        assert_eq!(alu.cmd("add y 6"), Ok(()));
        assert_eq!(alu.y, 6);
        
        assert_eq!(alu.cmd("div x y"), Ok(()));
        assert_eq!(alu.x, -7);
        assert_eq!(alu.y, 6);
        
        assert_eq!(alu.cmd("eql x -7"), Ok(()));
        assert_eq!(alu.x, 1);
    }
    
    #[test]
    fn test_program() {
        let program: &str = "\
            inp w\n\
            add z w\n\
            mod z 2\n\
            div w 2\n\
            add y w\n\
            mod y 2\n\
            div w 2\n\
            add x w\n\
            mod x 2\n\
            div w 2\n\
            mod w 2";
        
        let mut alu = ALU::default();
        alu.input.push(11);
        assert_eq!(alu.run(program), Ok(()));
        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 1);
    }
}