use crate::parser::Instruction;
use std::io::{self, Read, Write};
use anyhow::Result;

pub fn interpret(program: &[Instruction], debug: bool) -> Result<()> {
    let mut tape = vec![0u8; 30_000];
    let mut ptr = 0;
    execute(program, &mut tape, &mut ptr, debug) 
}

fn execute(program: &[Instruction], tape: &mut [u8], ptr: &mut usize, debug: bool) -> anyhow::Result<()> {
    for instr in program {
        match instr {
            Instruction::MoveRight => *ptr += 1,
            Instruction::MoveLeft => *ptr -= 1,
            Instruction::Increment => tape[*ptr] = tape[*ptr].wrapping_add(1),
            Instruction::Decrement => tape[*ptr] = tape[*ptr].wrapping_sub(1),
            Instruction::Output => {
                print!("{}", tape[*ptr] as char);
                io::stdout().flush()?;
            }
            Instruction::Input => {
                let mut buf = [0];
                io::stdin().read_exact(&mut buf)?;
                tape[*ptr] = buf[0];
            }
            Instruction::Loop(inner) => {
                while tape[*ptr] != 0 {
                    execute(inner, tape, ptr, debug)?;
                }
            }
        }
        if debug {
            print_debug_state(instr, tape, *ptr);
        }
    }
    Ok(())
}

fn print_debug_state(instr: &Instruction, tape: &[u8], ptr: usize) {
    println!("\n=== DEBUG ===");
    println!("Instruction: {:?}", instr);
    println!("Tape:       ");
    for (i, val) in tape.iter().take(20).enumerate() {
        if i == ptr {
            print!("[{:>3}] ", val);
        }
        else {
            print!("{:>3} ", val);
        }
    }
    println!("\nPointer:       {}", ptr);
    println!("=========\n");
}