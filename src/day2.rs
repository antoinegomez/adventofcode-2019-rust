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

pub fn prepare_and_execute(noun: usize, verb: usize) -> usize {
    let mut inp: Vec<usize> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0];
    inp[1] = noun;
    inp[2] = verb;
    let result = execute(inp);
    result
}

pub fn find_noun_and_verb(expected_result: usize) {
    let mut index_verb: usize = 0;
    let mut found = false;
    loop {
        let mut index_noun: usize = 0;
        loop {
            let result = prepare_and_execute(index_noun, index_verb);

            if result == expected_result {
                println!("noun: {}, verb: {}", index_noun, index_verb);
                println!("100 * noun + verb {}", (100 * index_noun) + index_verb);
                found = true;
                break;
            }

            if index_noun > 99 {
                break;
            }

            index_noun += 1;
        }

        if found || index_verb > 99 {
            break;
        }

        index_verb += 1;
    }

    if !found {
        println!("{}", "result not found");
    }
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
    inp[0]
}

#[test]
pub fn test_execute() {
    let inp: Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let result = execute(inp);
    assert_eq!(3500, result);
    println!("{}", result)
}

#[test]
pub fn test_part_2() {
    let result = prepare_and_execute(84, 78);
    assert_eq!(19690720, result);
}