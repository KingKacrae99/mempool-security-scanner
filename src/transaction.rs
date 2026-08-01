use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "TxId")]
    pub tx_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub sender: String,
    pub receiver: String,
    pub gasfee: u64,
}

impl Transaction {
    // Method to evaluate frontrunning threat
    pub fn analyze_security(&self) {
        if self.gasfee > 50 {
            println!(" ALERT!!! FRONTRUNNING ATTACK DETECTED!");
            println!("   Tx ID:    {}", self.tx_id);
            println!("   User:     {}", self.user_name);
            println!("   Route:    {} -> {}", self.sender, self.receiver);
            println!("   Gas Fee:  {} Gwei (High Spike)", self.gasfee);
            println!("--------------------------------------------------");
        } else {
            println!("✅ OK Tx {} ({} -> {}) - Gas: {}", self.tx_id, self.sender, self.receiver, self.gasfee);
        }
    }
}