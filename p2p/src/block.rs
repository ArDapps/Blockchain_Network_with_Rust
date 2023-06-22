use std::time::SystemTime;

pub struct Block{
    timestamp:u128,
    transactions:String,
    previous_block_hash:String,
    hash:String,
    height:usize,
    nonce:i32


}
pub struct  Blockchain {
    blocks: Vec<Block>
}

impl  Block {
    pub fn add_new_block(data:String,    previous_block_hash:String,height:usize)->Result<Block>{
        let time_stamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block= Block{
            timestamp:time_stamp,
          transactions:data,
            previous_block_hash,
            hash:String::new(),
            height,
            nonce:0
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self)-> Result<()> {
        info!("Block are Mining now");
        while !self.validate() {
            self.nonce +=1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())

    }

}