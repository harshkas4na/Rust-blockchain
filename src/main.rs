mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut my_blockchain = Blockchain::new();
    let mut count=0;
    loop{
        my_blockchain.add_block("First block after Genesis".to_string());
        my_blockchain.add_block("Second block after Genesis".to_string());
        my_blockchain.add_block("Third block after Genesis".to_string());

        count+=1;
        if count==3{
            break;
        }
    }
    

    for block in my_blockchain.chain.iter() {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", my_blockchain.is_valid());
}
