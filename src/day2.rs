use std::convert::{TryFrom, TryInto};

#[derive(PartialEq)]
enum OpCode {
    Add, Multiply, Halt
}

impl TryFrom<usize> for OpCode {
    type Error = String;

    fn try_from(item: usize) -> Result<OpCode, String> {
        use OpCode::*;

        match item {
            1 => Ok(Add),
            2 => Ok(Multiply),
            99 => Ok(Halt),
            _ => Err(String::from("Unknown OpCode encountered, stopping"))
        }
    }
}

pub fn prepare_and_execute() {
    let mut inp: Vec<usize> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0];
    inp[1] = 12;
    inp[2] = 2;
    let result = execute(inp);
    println!("{}", result);
}

fn execute(mut inp: Vec<usize>) -> usize {
    let mut sp = 0usize;

    loop {
        let op = inp[sp].try_into();
        match op {
            Ok(OpCode::Add) => {
                let p1 = inp[sp+1];
                let p2 = inp[sp+2];
                let p3 = inp[sp+3];
                inp[p3] = inp[p1] + inp[p2];
                sp += 4;
            },
            Ok(OpCode::Multiply) => {
                let p1 = inp[sp+1];
                let p2 = inp[sp+2];
                let p3 = inp[sp+3];
                inp[p3] = inp[p1] * inp[p2];
                sp += 4;
            },
            Ok(OpCode::Halt) => break,
            Err(error_message) => {
                println!("{}", error_message);
                break;
            }
        }
    }
    println!("{}", inp[0]);
    inp[0]
}

#[test]
pub fn test_example() {
    let inp: Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let result = execute(inp);
    assert_eq!(3501, result);
    println!("{}", result)
}