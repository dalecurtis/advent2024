#[derive(Debug, Default, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
    o: Vec<u64>,
    ip: u64,
}

type OperandFn = fn(&Registers) -> u64;
fn operand0(_reg: &Registers) -> u64 {
    0
}
fn operand1(_reg: &Registers) -> u64 {
    1
}
fn operand2(_reg: &Registers) -> u64 {
    2
}
fn operand3(_reg: &Registers) -> u64 {
    3
}
fn operand4(reg: &Registers) -> u64 {
    reg.a
}
fn operand5(reg: &Registers) -> u64 {
    reg.b
}
fn operand6(reg: &Registers) -> u64 {
    reg.c
}
fn operand7(_reg: &Registers) -> u64 {
    panic!("invalid")
}
const OPERATIONS: [OperandFn; 8] = [
    operand0, operand1, operand2, operand3, operand4, operand5, operand6, operand7,
];
const OPERATIONS_STR: [&str; 8] = ["0", "1", "2", "3", "a", "b", "c", "$$$$"];

type InstructionFn = fn(&mut Registers, u64);
fn adv_inst(reg: &mut Registers, combo: u64) {
    let imm = OPERATIONS[combo as usize](&reg);
    reg.a /= 2_u64.pow(imm as u32) as u64;
}
fn bxl_inst(reg: &mut Registers, imm: u64) {
    reg.b ^= imm as u64;
}
fn bst_inst(reg: &mut Registers, combo: u64) {
    let imm = OPERATIONS[combo as usize](&reg);
    reg.b = imm & 0b111;
}
fn jnz_inst(reg: &mut Registers, imm: u64) {
    if reg.a != 0 {
        reg.ip = imm / 2; // Since we pre-pair everything.
    }
}
fn bxc_inst(reg: &mut Registers, _imm: u64) {
    reg.b ^= reg.c;
}
fn out_inst(reg: &mut Registers, combo: u64) {
    let imm = OPERATIONS[combo as usize](&reg);
    reg.o.push(imm & 0b111);
}
fn bdv_inst(reg: &mut Registers, combo: u64) {
    let imm = OPERATIONS[combo as usize](&reg);
    reg.b = reg.a / 2_u64.pow(imm as u32) as u64;
}
fn cdv_inst(reg: &mut Registers, combo: u64) {
    let imm = OPERATIONS[combo as usize](&reg);
    reg.c = reg.a / 2_u64.pow(imm as u32) as u64;
}

const INSTRUCTIONS: [InstructionFn; 8] = [
    adv_inst, bxl_inst, bst_inst, jnz_inst, bxc_inst, out_inst, bdv_inst, cdv_inst,
];
const INSTRUCTIONS_STR: [&str; 8] = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];

fn dissassemble(instructions: &Vec<(u64, u64)>) {
    for inst in instructions {
        let ostr;
        match inst.0 {
            1 | 3 => {
                ostr = inst.1.to_string();
            }
            4 => {
                ostr = "_".to_string();
            }
            _ => {
                ostr = OPERATIONS_STR[inst.1 as usize].to_string();
            }
        }

        println!("\t{}\t{}", INSTRUCTIONS_STR[inst.0 as usize], ostr);
    }
}

fn beepboop(instructions: &Vec<(u64, u64)>, init_reg: &Registers) -> Registers {
    let mut reg = init_reg.clone();
    while reg.ip < instructions.len() as u64 {
        let (instruction, operand) = instructions[reg.ip as usize];
        let instruction_fn = INSTRUCTIONS[instruction as usize];
        let reg_a = reg.a;
        instruction_fn(&mut reg, operand);
        if instruction_fn != jnz_inst || reg_a == 0 {
            reg.ip += 1;
        }
    }
    return reg;
}

// Recursively solves the program.
fn solve_beepboop(
    instructions: &Vec<(u64, u64)>,
    init_reg: &Registers,
    program: &Vec<u64>,
    depth: usize,
) -> Registers {
    for i in 0..8 {
        let mut mod_reg = init_reg.clone();
        mod_reg.a += i as u64;

        let mut final_reg = beepboop(instructions, &mod_reg);
        if final_reg.o.len() > 0
            && final_reg.o.len() <= program.len()
            && final_reg.o == program[program.len() - final_reg.o.len()..]
        {
            if depth + 1 < program.len() {
                mod_reg.a *= 8;
                let solve_reg = solve_beepboop(instructions, &mod_reg, program, depth + 1);
                if solve_reg.o == *program {
                    return solve_reg;
                }
            } else {
                final_reg.a = mod_reg.a;
                return final_reg;
            }
        }
    }

    return init_reg.clone();
}

fn main() {
    // Sample program
    // let init_reg = Registers { a: 729, b: 0, c: 0, ip: 0 };
    // let program: Vec<_> = vec![0, 1, 5, 4, 3, 0];

    // Sample program 2
    // let init_reg = Registers { a: 0, b: 29, c: 0, ip: 0 };
    // let program: Vec<_> = vec![1,7];

    // Puzzle input
    let init_reg = Registers {
        a: 30886132,
        b: 0,
        c: 0,
        o: Vec::new(),
        ip: 0,
    };
    let program: Vec<_> = vec![2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 4, 5, 5, 3, 0];

    let instructions: Vec<_> = program
        .chunks(2)
        .map(|chunk| match chunk {
            &[a, b] => (a, b),
            _ => panic!("invalid input"),
        })
        .collect();

    {
        println!("initial_state={:?}", init_reg);
        let final_reg = beepboop(&instructions, &init_reg);
        println!("final_state={:?}", final_reg);
    }

    println!("disassembly:");
    dissassemble(&instructions);

    // Translated disassembly w/ some re-ordering for importance:
    //
    //   start:
    //     reg.b = reg.a & 0b111       // bst (4->a)
    //     reg.b = reg.b ^ 1           // bxl 1
    //     reg.c = reg.a / 2^reg.b     // cdv (5->b)
    //     reg.b = reg.b ^ 4           // bxl 4
    //     reg.b = reg.b ^ reg.c       // bxc (_)
    //     out(reg.b)                  // out (5->b)
    //
    //     reg.a = reg.a / 2^3         // adv 3
    //     jnz start
    let start_reg = Registers {
        a: 0,
        b: 0,
        c: 0,
        o: Vec::new(),
        ip: 0,
    };
    let solution = solve_beepboop(&instructions, &start_reg, &program, 0);
    println!("solution={:?}", solution);
}
