use super::*;

#[derive(Debug)]
pub enum LowLevelStatement {
    Set   (Register, Register), 
    Add   (Register, Register), 
    Addi  (Register, RegisterValueTypeSigned), 
    Sub   (Register, Register), 
    Mult  (Register, Register), 
    Multi (Register, RegisterValueTypeSigned), 
    Div   (Register, Register), 
    Divi  (Register, RegisterValueTypeSigned), 
    Je    (Register, Register, RegisterValueTypeSigned),
    Jne   (Register, Register, RegisterValueTypeSigned),
    Jmp   (RegisterValueTypeSigned),
}

#[derive(Debug)]
pub enum HighLevelStatement {
    VarStack (Variable),
    If,
    While,
    FunctionCall,
    Return (MyStruct),
}

#[derive(Debug)]
pub enum Statement {
    High (HighLevelStatement),
    Low (LowLevelStatement),
}

