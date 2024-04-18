use sha256::digest;

#[derive(Debug)]
struct Block {
    prev_hash: Option<String>,
    nonce: usize,
    value: String,
}

impl Block {
    fn hash(&self) -> String {
        digest(format!(
            "{}{}{}",
            self.prev_hash.as_ref().unwrap_or(&"".to_string()),
            self.nonce,
            self.value.to_owned()
        ))
    }
}
struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    fn new(value: &str) -> Self {
        Self {
            blocks: vec![Block {
                prev_hash: None,
                nonce: 1,
                value: value.to_string(),
            }],
        }
    }

    fn add_block(&mut self, value: &str) {
        let prev_hash = self.blocks.last().unwrap().hash();
        self.blocks.push(Block {
            prev_hash: Some(prev_hash),
            nonce: self.blocks.len() + 1,
            value: value.to_owned(),
        });
    }

    fn verify(&self) -> Result<(), &str> {
        let mut prev_hash = None;
        for (idx, block) in self.blocks.iter().enumerate() {
            println!("Block number {}, hash: {}", idx, block.hash());
            println!("  {:?}", block);
            if block.prev_hash.is_some() && block.prev_hash != prev_hash {
                return Err("Hash doesn't match. Tampered?");
            }
            prev_hash = Some(block.hash());
        }

        Ok(())
    }
}

fn main() {
    let mut chain = Chain::new("genesis block");

    chain.add_block(&"123");

    chain.verify().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_chain() {
        let mut chain = Chain::new("genesis block");

        chain.add_block(&"123");
        chain.add_block(&"456");
        chain.add_block(&"789");

        assert!(chain.verify().is_ok());
    }

    #[test]
    fn test_tamper() {
        let mut chain = Chain::new("genesis block");

        chain.add_block(&"123");
        chain.add_block(&"456");
        chain.add_block(&"789");

        chain.blocks[1].value = "foo".to_string();

        assert!(!chain.verify().is_ok());
    }
}
