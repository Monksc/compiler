use std::vec::Vec;

fn main() {

    let r1 = model::Register {
        rtype: model::RegisterType::Save,
        index: 0,
    };

    let r2 = model::Register {
        rtype: model::RegisterType::Save,
        index: 1,
    };

    let pc = model::Register {
        rtype: model::RegisterType::PC,
        index: 0,
    };

    let mut instructions = Vec::<model::Statement>::new();
    let add = model::Statement::Add(r1, r2);
    let addi1 = model::Statement::Addi(r1, 10);
    let addi2 = model::Statement::Addi(r1, 4);
    let subpc = model::Statement::Addi(pc, -4);
    instructions.push(add);
    instructions.push(addi1);
    instructions.push(addi2);
    instructions.push(subpc);

    let mut runner = interpreter::Runner::from(instructions);
    runner.run();
    runner.run();
    runner.run();
    runner.run();
    runner.run();
    runner.run();
    runner.run();
    
    println!("Hello, world! {:?}", runner);
}
