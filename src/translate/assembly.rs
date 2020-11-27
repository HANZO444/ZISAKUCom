use crate::x64;

pub fn to_assembly(codes: Vec<x64::Code>) -> String {
    let mut asm = ".global main\n".to_string();
    asm += "main:\n";

    for c in codes.iter() {
        asm += &format!("  {}\n", c.to_assembly());
    }
    asm += "  popq %rax\n";
    asm += "  ret\n";

    asm
}
