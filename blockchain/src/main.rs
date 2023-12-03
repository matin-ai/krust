mod models;
fn main() {
   // Create a variable to store the difficulty of the blockchain
   let difficulty = 1;
   // Create a new blockchain with the given difficulty
   let mut blockchain = models::blockchain::Blockchain::new(difficulty);
   // Add a block to the blockchain
   models::blockchain::Blockchain::add_block(&mut blockchain);
}