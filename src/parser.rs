#[derive(Debug)]

pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Instruction>),
}

pub fn parse(tokens: &[char]) -> anyhow::Result<(Vec<Instruction>, usize)> {
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            '>' => instructions.push(Instruction::MoveRight),
            '<' => instructions.push(Instruction::MoveLeft),
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => {
                let (loop_instructions, consumed) = parse(&tokens[i + 1..])?;
                instructions.push(Instruction::Loop(loop_instructions));
                i += consumed + 1;
            }
            ']' => return Ok((instructions, i)),
            _ => {}   
        }
        i += 1;
    }
    Ok((instructions, i))
}