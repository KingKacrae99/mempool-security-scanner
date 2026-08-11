use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::Database;
use serde::{Deserialize, Serialize};

// Primary Collection Schema: ThreatLogs
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatLog {
    pub tx_id: String,
    pub user_name: String,
    pub sender: String,
    pub receiver: String,
    pub gasfee: u64,
    pub status: String, 
}

// Related Collection Schema: Wallets
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletProfile {
    pub wallet_address: String,
    pub total_attacks: u64,
    pub threat_level: String,
}

// CREATE (Insert Threat & Update Linked Wallet) 
pub async fn log_threat(db: &Database, threat: ThreatLog) -> Result<(), Box<dyn std::error::Error>> {
    let threats_col = db.collection::<ThreatLog>("threats");
    let wallets_col = db.collection::<WalletProfile>("wallets");

    // Save threat log
    let sender_address = threat.sender.clone();
    threats_col.insert_one(threat).await?;
    println!("✅ Threat logged to SecurityDB 'threats' collection.");

    // Update or Create (Upsert) related Wallet Profile
    let wallet_filter = doc! { "wallet_address": &sender_address };
    let wallet_update = doc! {
        "$inc": { "total_attacks": 1 },
        "$set": { "threat_level": "HIGH" }
    };
    let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
    wallets_col.update_one(wallet_filter, wallet_update).await?;
    println!("🔗 Related Wallet profile updated in 'wallets' collection.");

    Ok(())
}

// READ (Query & Filter) 
pub async fn get_high_gas_threats(db: &Database, min_gas: u64) -> Result<Vec<ThreatLog>, Box<dyn std::error::Error>> {
    let collection = db.collection::<ThreatLog>("threats");
    let filter = doc! { "gasfee": { "$gt": min_gas as i64 } };

    let mut cursor = collection.find(filter).await?;
    let mut threats = Vec::new();

    while let Some(threat) = cursor.try_next().await? {
        threats.push(threat);
    }

    Ok(threats)
}

// UPDATE (Modify Status)
pub async fn update_threat_status(db: &Database, tx_id: &str, new_status: &str) -> Result<(), Box<dyn std::error::Error>> {
    let collection = db.collection::<ThreatLog>("threats");
    let filter = doc! { "tx_id": tx_id };
    let update = doc! { "$set": { "status": new_status } };

    collection.update_one(filter, update).await?;
    println!("Threat {} status updated to '{}'.", tx_id, new_status);
    Ok(())
}

// DELETE (Remove)
pub async fn delete_threat(db: &Database, tx_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let collection = db.collection::<ThreatLog>("threats");
    let filter = doc! { "tx_id": tx_id };

    let result = collection.delete_one(filter).await?;
    if result.deleted_count > 0 {
        println!("Threat {} deleted successfully.", tx_id);
    } else {
        println!("⚠️ No threat found with Tx ID: {}", tx_id);
    }

    Ok(())
}