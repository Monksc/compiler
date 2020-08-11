use super::*;

pub struct SmallBlock {
    fcall: Function,
    params: Vec<SmallBlock>,
}

impl SmallBlock {
    pub fn is_valid(&self) -> bool {

        // make sure there the corect len
        if self.params.len() != self.fcall.params.len() {
            return false;
        }

        for i in 0..self.params.len() {
            // param i has correct type
            if self.fcall.params[i] != 
                self.params[i].fcall.return_type {
                return false;
            }

            // param i is valid 
            if !self.params[i].is_valid() {
                return false;
            }
        }

        // everything is valid
        true
    }

    pub fn total_bytes(&self) -> u32 {

        let mut size = 0;

        for p in &self.params {
            size += p.total_bytes();
        }

        return size;
    }
}
