use bitcoin::{Opcode, ScriptBuf};

#[derive(Clone, Debug)]
enum Block {
    Call(BetterScript),
    Script(ScriptBuf),
}

impl Block {
    fn new_script() -> Self {
        let buf = ScriptBuf::new();
        Block::Script(buf)
    }
}

#[derive(Clone, Debug)]
pub struct BetterScript {
    size: usize,
    blocks: Vec<Block>,
}

impl BetterScript {
    pub fn new() -> Self {
        let blocks = Vec::new();
        BetterScript { size: 0, blocks }
    }

    fn get_script_block(&mut self) -> &mut ScriptBuf {
        // Check if the last block is a Script block
        let is_script_block = match self.blocks.last_mut() {
            Some(Block::Script(_)) => true,
            _ => false,
        };
        
        // Create a new Script block if necessary
        if !is_script_block {
            self.blocks.push(Block::new_script());
        }

        if let Some(Block::Script(ref mut script)) = self.blocks.last_mut() {
            script
        } else {
            unreachable!()
        }
    }

    pub fn push_opcode(&mut self, data: Opcode) {
        self.size += 1;
        let script = self.get_script_block();
        script.push_opcode(data);
    }
    
    pub fn push_environment_script(&mut self, data: BetterScript) {
        self.size += data.size;
        self.blocks.push(Block::Call(data));
    }

    pub fn compile_to_bytes(&self, script: &mut Vec<u8>) {
        for block in self.blocks.as_slice() {
            match block {
                Block::Call(call) => call.compile_to_bytes(script),
                Block::Script(block_script) => script.extend(block_script.as_bytes()),
            }
        };
    }

    pub fn compile(self) -> ScriptBuf {
        let mut script = Vec::with_capacity(self.size);
        self.compile_to_bytes(&mut script);
        ScriptBuf::from_bytes(script)
    }
}


#[cfg(test)]
mod tests {
    use bitcoin::opcodes::{all::*, OP_TRUE};

    use super::*;

    #[test]
    fn test_compile() {
        let mut script = BetterScript::new();
        script.push_opcode(OP_TRUE);
        script.push_opcode(OP_ADD);
        script.push_opcode(OP_DUP);
        script.push_opcode(OP_ADD);
        script.push_opcode(OP_DUP);
        script.push_opcode(OP_ADD);
        for _ in 0..10 {
            script.push_environment_script(script.clone());
        }
        let sub_script = script.clone();

        for _ in 0..80000 {
            script.push_environment_script(sub_script.clone());
        }
        println!("size: {}", script.size);
        println!("size GB: {}", script.size as f64 / 1_000_000_000.0);

        let compiled_script = script.compile();
        println!("compiled size (bytes): {}", compiled_script.len());
        assert_eq!(1, 4);
    }
}
