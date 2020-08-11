pub type MyStructSize = u8;

pub type Variable = (String, MyStruct);

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MyStruct {
    name: String,
    variables: Vec<Variable>,
    size: Option<MyStructSize>,
}

impl MyStruct {
    pub fn size(&self) -> u8 {
        if let Some(size) = self.size {
            return size;
        }

        let mut size : MyStructSize = 0;

        for (_, s) in &self.variables {
            size += s.size();
        }

        return size;
    }
}

