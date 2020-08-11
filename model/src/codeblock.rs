use super::*;

pub struct CodeBlock {
    lines: Vec<SmallBlock>
}

impl CodeBlock {
    pub fn total_bytes(&self) -> u32 {

        let mut size = 0;

        for p in &self.lines {
            size += p.total_bytes();
        }

        return size;
    }
}
