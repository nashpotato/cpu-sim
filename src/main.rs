pub mod cpu;
pub mod util;
fn main() {
    let mut cpu = cpu::cpu::CPU::new();
    cpu.fdew();
    let parse = util::util::parseFile("tests/arith_mov");
    match parse{
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }
}
