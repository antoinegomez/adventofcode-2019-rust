extern crate array_tool;

use std::convert::{TryFrom};
use array_tool::vec::Shift;
use std::io;
use std::fmt::Debug;

#[derive(PartialEq)]
enum OpCode {
    Add, Multiply, Set, Get, Halt, JumpIfTrue, JumpIfFalse, LessThan, Equals
}

#[derive(PartialEq, Debug)]
enum Mode {
    Position, Immediate
}

struct OpCodeAndModes {
    op_code: OpCode,
    modes: Vec<Mode>
}

impl TryFrom<i32> for OpCode {
    type Error = String;

    fn try_from(item: i32) -> Result<OpCode, String> {
        use OpCode::*;

        match item {
            1 => Ok(Add),
            2 => Ok(Multiply),
            3 => Ok(Set),
            4 => Ok(Get),
            5 => Ok(JumpIfTrue),
            6 => Ok(JumpIfFalse),
            7 => Ok(LessThan),
            8 => Ok(Equals),
            99 => Ok(Halt),
            _ => Err(String::from(format!("Unknown OpCode encountered, stopping: {:?}", item)))
        }
    }
}

impl TryFrom<u32> for Mode {
    type Error = String;

    fn try_from(item: u32) -> Result<Mode, String> {
        use Mode::*;

        match item {
            0 => Ok(Position),
            1 => Ok(Immediate),
            _ => Err(String::from(format!("Unknown Mode encountered: {:?}", item)))
        }
    }
}

pub fn main() {
    let result = prepare_and_execute();
}

fn prepare_and_execute() -> i32 {
    let inp: Vec<i32> = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1102,59,58,224,1001,224,-3422,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,59,30,225,1101,53,84,224,101,-137,224,224,4,224,1002,223,8,223,101,3,224,224,1,223,224,223,1102,42,83,225,2,140,88,224,1001,224,-4891,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,61,67,225,101,46,62,224,1001,224,-129,224,4,224,1002,223,8,223,101,5,224,224,1,223,224,223,1102,53,40,225,1001,35,35,224,1001,224,-94,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1101,5,73,225,1002,191,52,224,1001,224,-1872,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,102,82,195,224,101,-738,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1101,83,52,225,1101,36,77,225,1101,9,10,225,1,113,187,224,1001,224,-136,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1007,226,226,224,1002,223,2,223,1006,224,329,1001,223,1,223,1108,226,226,224,102,2,223,223,1006,224,344,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,359,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,389,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,404,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,419,101,1,223,223,1008,226,677,224,1002,223,2,223,1006,224,434,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,8,226,226,224,1002,223,2,223,1006,224,479,1001,223,1,223,107,226,677,224,102,2,223,223,1005,224,494,1001,223,1,223,7,226,226,224,102,2,223,223,1005,224,509,1001,223,1,223,107,226,226,224,102,2,223,223,1005,224,524,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,539,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,554,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,569,101,1,223,223,108,226,677,224,1002,223,2,223,1006,224,584,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,614,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,659,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226];
    let result = execute(inp);
    result
}

fn get_operation_with_mode(code: i32) -> Result<OpCodeAndModes, String> {
    let mut operation_vec: Vec<u32> = code.to_string().chars().map(|w| w.to_digit(10).unwrap()).collect();

    if operation_vec.len() > 5 {
      return Err(String::from("Invalid OpCode sequence"))
    } else if operation_vec.len() < 3 {
        let op_code = OpCode::try_from(code);

        if op_code.is_err() {
            return Err(format!("Invalid op code: {:?}", code));
        }

        return Ok(OpCodeAndModes {
            op_code: OpCode::try_from(code).unwrap(),
            modes: vec![Mode::Position, Mode::Position, Mode::Position]
        })
    } else if operation_vec.len() == 4 {
        operation_vec.unshift(0);
    } else if operation_vec.len() == 3 {
        operation_vec.unshift(0);
        operation_vec.unshift(0);
    }

    let operation = operation_vec.pop().unwrap();
    operation_vec.pop();
    let op_code = OpCode::try_from(operation as i32);

    if op_code.is_err() {
        return Err(format!("Invalid op code: {:?}", code));
    }

    Ok(OpCodeAndModes {
        op_code: op_code.unwrap(),
        modes: operation_vec.iter().map(|&w| Mode::try_from(w).unwrap()).collect()
    })
}

fn get_position_value(inp: &Vec<i32>, modes: &Vec<Mode>, position: usize, sp: &usize) -> Result<i32, String> {
    let mode: &Mode;
    let index = inp[position + *sp];

    if position == 1 {
        mode = &modes[2];
    } else if position == 2 {
        mode = &modes[1];
    } else if position == 3 {
        mode = &modes[0];
    } else {
        return Err(String::from("Invalid position"))
    }

    match mode {
        Mode::Position => {
            Ok(inp[index as usize])
        },
        Mode::Immediate => {
            Ok(index)
        }
    }
}

/// Reads a number from STDIN
fn read_stdin() -> isize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("STDIN read failed.");
    buffer.trim().parse::<isize>().unwrap()
}

/// Write a number to STDOUT
fn write_stdout(number: isize) {
    println!("{}", number);
}

fn execute(mut inp: Vec<i32>) -> i32 {
    let mut sp = 0usize;

    loop {
        let op_and_modes = get_operation_with_mode(inp[sp]);
        let op_and_modes_unwrapped = op_and_modes.unwrap();
        let op = op_and_modes_unwrapped.op_code;
        let modes = op_and_modes_unwrapped.modes;

        match op {
            OpCode::Add => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                let p3 = inp[sp+3] as usize;
                println!("Add {} + {} to {}", p1, p2, p3);
                println!("args {},{},{},{}", inp[sp], inp[sp + 1], inp[sp + 2], inp[sp + 3]);
                inp[p3] = p1 + p2;
                sp += 4;
            },
            OpCode::Multiply => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                let p3 = inp[sp + 3] as usize;
                println!("Multiply {} + {} to {}", p1, p2, p3);
                inp[p3] = p1 * p2;
                sp += 4;
            },
            OpCode::Get => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap() as isize;
                println!("Get {}", p1);
                write_stdout(p1);
                sp += 2;
            },
            OpCode::Set => {
                let p1 = inp[sp + 1] as usize;
                println!("Set {}", p1);
                inp[p1] = read_stdin() as i32;
                sp += 2;
            },
            OpCode::JumpIfTrue => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                println!("Jump to {} if {} is non-zero", p1, p2);

                if p1 > 0 {
                    sp = p2 as usize;
                } else {
                    sp += 3;
                }
            },
            OpCode::JumpIfFalse => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                println!("Jump to {} if {} is zero", p1, p2);

                if p1 == 0 {
                    sp = p2 as usize;
                } else {
                    sp += 3;
                }
            },
            OpCode::LessThan => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                let p3 = inp[sp + 3] as usize;
                println!("Set {} to 1 if {} is less than {}", p3, p1, p2);

                if p1 < p2 {
                    inp[p3] = 1;
                } else {
                    inp[p3] = 0;
                }

                sp += 4;
            },
            OpCode::Equals => {
                let p1 = get_position_value(&inp, &modes, 1, &sp).unwrap();
                let p2 = get_position_value(&inp, &modes, 2, &sp).unwrap();
                let p3 = inp[sp + 3] as usize;
                println!("Set {} to 1 if {} is equals to {}", p3, p1, p2);

                if p1 == p2 {
                    inp[p3] = 1;
                } else {
                    inp[p3] = 0;
                }

                sp += 4;
            }
            OpCode::Halt => break
        }
    }
    inp[0]
}

#[test]
pub fn test_execute() {
    let inp: Vec<i32> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let result = execute(inp);
    assert_eq!(3500, result);
    println!("{}", result)
}
