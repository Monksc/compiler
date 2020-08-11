
pub mod statement;
pub use statement::LowLevelStatement;

pub mod mystruct;
pub use mystruct::MyStruct;
pub use mystruct::Variable;

pub mod register;
pub use register::Register;
pub use register::RegisterType;
pub use register::RegisterValueTypeSigned;
//pub use register::RegisterValueTypeUnSigned;

pub mod ifstatement;

pub mod function;
pub use function::Function;

pub mod smallblock;
pub use smallblock::SmallBlock;

pub mod codeblock;
pub use codeblock::CodeBlock;

