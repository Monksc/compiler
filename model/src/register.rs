
pub type RegisterValueTypeSigned = i32;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
pub enum RegisterType {
    Save,
    Temporary,
    Arguments,
    Return,
    PC
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
pub struct Register {
    pub rtype: RegisterType,
    pub index: u8,
}


