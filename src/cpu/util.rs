use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
use std::vec::Vec;
use std::string::String;
use super::cpu::INSTR;

fn to_instr(v: &mut Vec<&str>) -> Result<INSTR, String>{
    let op = v[0];
    match op{
        "mov" | "movi" | "addi" | "subi" =>{
            let arg2 = v.pop();
            let arg1 = v.pop();
            match (arg1, arg2){
                (Some(x), Some(y)) =>{
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val  = y[1..].to_string().parse::<u32>().unwrap();
                    if op == "mov" {
                        Ok(INSTR::MOV(dest, val))
                    }
                    else if op == "movi" {
                        Ok(INSTR::MOVI(dest, val))
                    }
                    else if op == "addi"{
                        Ok(INSTR::ADDI(dest, val))
                    }
                    else {
                        Ok(INSTR::SUBI(dest, val))
                    }
                }
                _ => Err(format!("Not enough arguments to instr = {}", op)),
            }
        }
        "add" | "addr" | "sub" | "subr" => {
            let arg3 = v.pop();
            let arg2 = v.pop();
            let arg1 = v.pop();
            match (arg1, arg2, arg3){
                (Some(x), Some(y), Some(z)) => {
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val1 = y[1..].to_string().parse::<u32>().unwrap();
                    let val2 = z[1..].to_string().parse::<u32>().unwrap();
                    if op == "add"{
                        Ok(INSTR::ADD(dest, val1, val2))
                    }
                    else if op == "sub"{
                        Ok(INSTR::SUB(dest, val1, val2))
                    }
                    else if op == "addr" {
                        Ok(INSTR::ADDR(dest, val1, val2))
                    }
                    else {
                        Ok(INSTR::SUBR(dest, val1, val2))
                    }
                },
                _ => Err(format!("Not enough arguments to instr = {}", op)),
            }
        }
        _ => Err(format!("can't read instruction = {}", op)),
    }
}

pub fn parse_file(file_path: &'static str ) -> io::Result<Vec<Result<INSTR, String>>> {
    let file = File::open(file_path);
    let mut instr= Vec::new();
    match file {
        Ok(f) =>{
            let reader = BufReader::new(f);
            for split in reader.lines(){
                match split{
                    Ok(s) => {
                        let mut collect = s.split(' ').collect();
                        instr.push(to_instr(&mut collect));
                    },
                    Err(_) => (),
                }
            }
            Ok(instr)
        },
        _ => Err(Error::new(ErrorKind::Other, format!("FAILED TO OPEN FILE {}", file_path))),
    }
}

#[cfg(test)]
mod tests{
    use crate::cpu::util::*;
    use crate::cpu::cpu::INSTR;

    #[test]
    fn parse_mov() {
        let mut instr = vec!["mov", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::MOV(0, 1)) => (),
            _ => panic!(format!("Failed to parse MOV instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_movi() {
        let mut instr = vec!["movi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::MOVI(0, 1)) => (),
            _ => panic!(format!("Failed to parse MOVI instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_addi() {
        let mut instr = vec!["addi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::ADDI(0, 1)) => (),
            _ => panic!(format!("Failed to parse ADDi instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_subi() {
        let mut instr = vec!["subi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::SUBI(0, 1)) => (),
            _ => panic!(format!("Failed to parse SUBI instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_add() {
        let mut instr = vec!["add", "r0", "#1", "#2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::ADD(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse ADD instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_addr() {
        let mut instr = vec!["addr", "r0", "r1", "r2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::ADDR(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse ADDR instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_sub() {
        let mut instr = vec!["sub", "r0", "#1", "#2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::SUB(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse SUB instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_subr() {
        let mut instr = vec!["subr", "r0", "r1", "r2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(INSTR::SUBR(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse SUBR instruction = {:?}", instr))
        }
    }

    #[test]
    fn parse_src_file() {
        let res = parse_file("./src/programs/arith_mov");
        let expected = vec![
            INSTR::MOVI(0, 42),
            INSTR::MOV(0, 0),
            INSTR::ADDR(0, 0, 0),
            INSTR::ADDI(0, 42),
            INSTR::ADD(0, 42, 42),
            INSTR::SUBR(0, 0, 0),
            INSTR::SUBI(0, 42),
            INSTR::SUB(0, 42, 42)
        ];
        match res {
            Ok(v) => {
                assert_eq!(v.len(), expected.len());
                for (l, r) in expected.into_iter().zip(v){
                    match r{
                        Ok(instr) => assert_eq!(l, instr),
                        Err(e) => panic!("Failed to parse string {}", e),
                    }
                }
            }
            Err(e) => panic!(e.to_string()),
        }
    }
}
