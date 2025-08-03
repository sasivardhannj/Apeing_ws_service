use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenEvent {
    pub event_type: String,
    pub timestamp: String,
    pub transaction_signature: String,
    pub token: TokenDetails,
    pub pump_data: PumpData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDetails {
    pub mint_address: String,
    pub name: String,
    pub symbol: String,
    pub creator: String,
    pub supply: u64,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpData {
    pub bonding_curve: String,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}

pub fn parse_event(raw_message: &str) -> Option<String> {
    let parsed: Value = serde_json::from_str(raw_message).ok()?;
    
    // Check if this is a program notification (account change)
    if parsed["method"] != "programNotification" {
        return None;
    }
    
    // Extract account data from the notification
    let account_data = &parsed["params"]["result"]["value"];
    let pubkey = account_data["pubkey"].as_str()?;
    let slot = parsed["params"]["result"]["context"]["slot"].as_u64()?;
    
    // Check if this is a pump.fun program account change
    if let Some(account) = account_data["account"].as_object() {
        let owner = account["owner"].as_str()?;
        
        // Only process pump.fun program account changes
        if owner == "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" {
            return extract_pump_fun_account_data(pubkey, account, slot);
        }
    }
    
    None
}

fn extract_pump_fun_account_data(pubkey: &str, account: &serde_json::Map<String, Value>, slot: u64) -> Option<String> {
    // Generate a mock transaction signature based on the pubkey and slot
    let transaction_signature = format!("{}_{}", pubkey[..8].to_string(), slot);
    
    // Extract data from account (this is a simplified example since we don't have the exact data structure)
    // In a real implementation, you would decode the base64 data according to pump.fun's account structure
    let token_details = TokenDetails {
        mint_address: pubkey.to_string(),
        name: format!("Token_{}", &pubkey[..8]),
        symbol: "MTK".to_string(),
        creator: "DEF456...".to_string(),
        supply: 1_000_000_000,
        decimals: 6,
    };
    
    let pump_data = PumpData {
        bonding_curve: "GHI789...".to_string(),
        virtual_sol_reserves: 30_000_000_000,
        virtual_token_reserves: 1_073_000_000_000_000,
    };
    
    let event = TokenEvent {
        event_type: "token_created".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        transaction_signature,
        token: token_details,
        pump_data,
    };
    
    serde_json::to_string(&event).ok()
}

fn extract_pump_fun_data(log_message: &str, signature: String, slot: u64) -> Option<String> {
    // Extract mint address from logs (this is a simplified example)
    let mint_address = extract_mint_address(log_message)
        .unwrap_or_else(|| format!("mint_{}", slot));
    
    // Extract other token details (simplified parsing)
    let token_details = TokenDetails {
        mint_address: mint_address.clone(),
        name: extract_token_name(log_message).unwrap_or_else(|| "Unknown Token".to_string()),
        symbol: extract_token_symbol(log_message).unwrap_or_else(|| "UNK".to_string()),
        creator: extract_creator(log_message).unwrap_or_else(|| "unknown".to_string()),
        supply: extract_supply(log_message).unwrap_or(1_000_000_000),
        decimals: extract_decimals(log_message).unwrap_or(6),
    };
    
    let pump_data = PumpData {
        bonding_curve: extract_bonding_curve(log_message).unwrap_or_else(|| "curve_unknown".to_string()),
        virtual_sol_reserves: extract_virtual_sol_reserves(log_message).unwrap_or(30_000_000_000),
        virtual_token_reserves: extract_virtual_token_reserves(log_message).unwrap_or(1_073_000_000_000_000),
    };
    
    let event = TokenEvent {
        event_type: "token_created".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        transaction_signature: signature,
        token: token_details,
        pump_data,
    };
    
    serde_json::to_string(&event).ok()
}

// Helper functions to extract data from log messages
fn extract_mint_address(log: &str) -> Option<String> {
    // Look for patterns like "Program log: Mint: ABC123..."
    if let Some(start) = log.find("Mint: ") {
        let after_mint = &log[start + 6..];
        if let Some(end) = after_mint.find(' ') {
            return Some(after_mint[..end].to_string());
        }
    }
    None
}

fn extract_token_name(log: &str) -> Option<String> {
    // Look for patterns like "Name: MyToken"
    if let Some(start) = log.find("Name: ") {
        let after_name = &log[start + 6..];
        if let Some(end) = after_name.find(' ') {
            return Some(after_name[..end].to_string());
        }
    }
    None
}

fn extract_token_symbol(log: &str) -> Option<String> {
    // Look for patterns like "Symbol: MTK"
    if let Some(start) = log.find("Symbol: ") {
        let after_symbol = &log[start + 8..];
        if let Some(end) = after_symbol.find(' ') {
            return Some(after_symbol[..end].to_string());
        }
    }
    None
}

fn extract_creator(log: &str) -> Option<String> {
    // Look for patterns like "Creator: DEF456..."
    if let Some(start) = log.find("Creator: ") {
        let after_creator = &log[start + 9..];
        if let Some(end) = after_creator.find(' ') {
            return Some(after_creator[..end].to_string());
        }
    }
    None
}

fn extract_supply(log: &str) -> Option<u64> {
    // Look for patterns like "Supply: 1000000000"
    if let Some(start) = log.find("Supply: ") {
        let after_supply = &log[start + 8..];
        if let Some(end) = after_supply.find(' ') {
            return after_supply[..end].parse::<u64>().ok();
        }
    }
    None
}

fn extract_decimals(log: &str) -> Option<u8> {
    // Look for patterns like "Decimals: 6"
    if let Some(start) = log.find("Decimals: ") {
        let after_decimals = &log[start + 10..];
        if let Some(end) = after_decimals.find(' ') {
            return after_decimals[..end].parse::<u8>().ok();
        }
    }
    None
}

fn extract_bonding_curve(log: &str) -> Option<String> {
    // Look for patterns like "BondingCurve: GHI789..."
    if let Some(start) = log.find("BondingCurve: ") {
        let after_curve = &log[start + 13..];
        if let Some(end) = after_curve.find(' ') {
            return Some(after_curve[..end].to_string());
        }
    }
    None
}

fn extract_virtual_sol_reserves(log: &str) -> Option<u64> {
    // Look for patterns like "VirtualSolReserves: 30000000000"
    if let Some(start) = log.find("VirtualSolReserves: ") {
        let after_reserves = &log[start + 20..];
        if let Some(end) = after_reserves.find(' ') {
            return after_reserves[..end].parse::<u64>().ok();
        }
    }
    None
}

fn extract_virtual_token_reserves(log: &str) -> Option<u64> {
    // Look for patterns like "VirtualTokenReserves: 1073000000000000"
    if let Some(start) = log.find("VirtualTokenReserves: ") {
        let after_reserves = &log[start + 22..];
        if let Some(end) = after_reserves.find(' ') {
            return after_reserves[..end].parse::<u64>().ok();
        }
    }
    None
}