use mongodb::Database;
use serde::Deserialize;
use crate::securitydb::{self, ThreatLog};

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
    // Evaluates transaction and persists threat if gas fee exceeds threshold
    pub async fn analyze_security(&self, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
        if self.gasfee > 50 {
            println!("🚨 [ALERT] FRONTRUNNING ATTACK DETECTED!");
            println!("   Tx ID:   {}", self.tx_id);
            println!("   User:    {}", self.user_name);
            println!("   Route:   {} -> {}", self.sender, self.receiver);
            println!("   Gas Fee: {} Gwei (High Spike)", self.gasfee);
            println!("--------------------------------------------------");

            // Construct ThreatLog document
            let threat = ThreatLog {
                tx_id: self.tx_id.clone(),
                user_name: self.user_name.clone(),
                sender: self.sender.clone(),
                receiver: self.receiver.clone(),
                gasfee: self.gasfee,
                status: "FLAGGED".to_string(),
            };

            // Call securitydb module to persist to MongoDB Atlas
            securitydb::log_threat(db, threat).await?;
        } else {
            println!("✅ [OK] Tx {} ({} -> {}) - Gas: {}", self.tx_id, self.sender, self.receiver, self.gasfee);
        }

        Ok(())
    }
}