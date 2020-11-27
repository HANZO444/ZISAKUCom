#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Code {
    PushInt128 { value: i128 },
    PushRAX,
    PopRAX,
    PopRDI,
    AddRDIToRAX,
    SubRDIFromRAX,
}

impl Code {
    pub fn to_assembly(&self) -> String {
        match self {
            Code::PushRAX => "pushq %rax".to_string(),
            Code::PushInt128 { value } => format!("pushq ${}", value),
            Code::PopRDI => "pop %rdi".to_string(),
            Code::PopRAX => "pop %rax".to_string(),
            Code::AddRDIToRAX => "addq %rdi, %rax".to_string(),
            Code::SubRDIFromRAX => "subq %rdi, %rax".to_string(),
        }
    }
}
