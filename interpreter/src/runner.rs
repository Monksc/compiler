use std::collections::HashMap;

#[derive(Debug)]
pub struct Runner {
    registers: HashMap<model::Register, model::RegisterValueTypeSigned>,
    statements: Vec<model::Statement>,
}

impl Runner {

    fn pc_reg() -> model::Register {
        model::Register {
            rtype: model::RegisterType::PC,
            index: 0,
        }
    }

    pub fn from(statement: Vec<model::Statement>) -> Runner{

        let mut regs = HashMap::new();

        for i in 0..8 {

            let new_reg = model::Register {
                rtype: model::RegisterType::Save,
                index: i,
            };

            let value : model::RegisterValueTypeSigned = 0;
            regs.insert(new_reg, value);
        }

        let pc = Runner::pc_reg();

        let value : model::RegisterValueTypeSigned = 0;
        regs.insert(pc, value);

        Runner {
            registers: regs,
            statements: statement,
        }
    }

    pub fn run(&mut self) {
        self.step();
    }

    fn step(&mut self) {

        let register = model::Register {
            rtype: model::RegisterType::PC,
            index: 0,
        };

        let index = self.registers.get_mut(&register).unwrap();
        let statement = &self.statements[*index as usize];

        *index += 1;

        println!("{:?}", statement);
        match statement {
            model::Statement::Set (reg1, reg2) => {
                let v2 = self.registers[reg2];
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 = v2;
            },
            model::Statement::Add (reg1, reg2) => {
                let v2 = self.registers[reg2];
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 += v2;
            },
            model::Statement::Addi (reg1, value) => {
                let rvalue = self.registers.get_mut(reg1).unwrap();
                *rvalue += value
            },
            model::Statement::Sub (reg1, reg2) => {
                let v2 = self.registers[reg2];
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 -= v2;
            },
            model::Statement::Mult (reg1, reg2) => {
                let v2 = self.registers[reg2];
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 *= v2;
            },
            model::Statement::Multi (reg1, value) => {
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 *= value;
            },
            model::Statement::Div (reg1, reg2) => {
                let v2 = self.registers[reg2];
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 /= v2;
            },
            model::Statement::Divi (reg1, value) => {
                let v1 = self.registers.get_mut(reg1).unwrap();
                *v1 /= value;
            },
            model::Statement::Je (reg1, reg2, dx) => {
                let v1 = self.registers[reg1];
                let v2 = self.registers[reg2];
                if v1 == v2 {
                    let pc = self.registers.get_mut(&Runner::pc_reg()).unwrap();
                    *pc += dx;
                }
            },
            model::Statement::Jne (reg1, reg2, dx) => {
                let v1 = self.registers[reg1];
                let v2 = self.registers[reg2];
                if v1 != v2 {
                    let pc = self.registers.get_mut(&Runner::pc_reg()).unwrap();
                    *pc += dx;
                }
            },
            model::Statement::Jmp (dx) => {
                let pc = self.registers.get_mut(&Runner::pc_reg()).unwrap();
                *pc += dx;
            },
            _ => {
                println!("did not implement that instruction yet");
            }
        }
    }

}

