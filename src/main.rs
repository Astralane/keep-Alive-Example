use base64::Engine;
use rand::Rng;
use serde_json::{Value, json};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{EncodableKey, Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey, system_instruction};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::Instant;

const ASTRALANE_TIP_ADDR: &str = "astra4uejePWneqNaJKuFFA8oonqCE1sqF6b45kDMZm";
const ASTRALANE_TIP_AMOUNT: u64 = 1_000_000;
const ASTRALANE_URL: &str = "http://fr.gateway.astralane.io/iris";
const ASTRALANE_API_KEY: &str = "VERY VERY SECRET";

const SOLANA_MAINNET_URL: &str = "https://api.mainnet-beta.solana.com";
const SOLANA_MEMO_PROGRAM: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

const SIGNER_KEYPAIR_FILE_PATH: &str = "/lujing/dao/id.json";
const COMPUTE_UNIT_LIMIT: u32 = 10000;
const COMPUTE_UNIT_PRICE: u64 = 10000;

pub fn create_random_memo_instruction(signer: Pubkey) -> Instruction {
    let random_string = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();

    Instruction {
        accounts: vec![AccountMeta::new(signer, true)],
        program_id: SOLANA_MEMO_PROGRAM,
        data: random_string.as_bytes().to_vec(),
    }
}

async fn txn_payload_builder(
    keypair: &Keypair,
    recent_blockhash: Hash,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut instructions = Vec::new();

    let compute_unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(COMPUTE_UNIT_LIMIT);
    instructions.push(compute_unit_limit);

    let compute_unit_price = ComputeBudgetInstruction::set_compute_unit_price(COMPUTE_UNIT_PRICE);
    instructions.push(compute_unit_price);

    let memo_instruction = create_random_memo_instruction(keypair.pubkey());
    instructions.push(memo_instruction);

    let tip_instruction = system_instruction::transfer(
        &keypair.pubkey(),
        &Pubkey::from_str(ASTRALANE_TIP_ADDR).unwrap(),
        ASTRALANE_TIP_AMOUNT,
    );
    instructions.push(tip_instruction);

    let message = Message::new(&instructions, Some(&keypair.pubkey()));

    let txn = Transaction::new(&[&keypair], message, recent_blockhash);

    let tx_bytes = bincode::serialize(&txn)?;
    let encoded_transaction = base64::prelude::BASE64_STANDARD.encode(tx_bytes);

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": [encoded_transaction, {
            "encoding": "base64",
            "skipPreflight": true
        }]
    });

    Ok(body)
}

async fn send_txn_with_keep_alive() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .pool_idle_timeout(Some(Duration::from_secs(85))) // Keep connections alive for 85
        .build()?;

    let solana_client = RpcClient::new_with_commitment(
        SOLANA_MAINNET_URL.parse().unwrap(),
        CommitmentConfig::finalized(),
    );
    let keypair = Keypair::read_from_file(SIGNER_KEYPAIR_FILE_PATH)?;

    let recent_blockhash = solana_client.get_latest_blockhash().await?;

    let body = txn_payload_builder(&keypair, recent_blockhash).await?;
    let time = Instant::now();
    let _res = client
        .post(ASTRALANE_URL)
        .header("api_key", ASTRALANE_API_KEY)
        .json(&body)
        .send()
        .await?;
    let elapsed = time.elapsed();
    println!("Total time after first request: {}ms", elapsed.as_millis());

    tokio::time::sleep(Duration::from_secs(5)).await;
    let _res = client
        .get(format!("{}/gethealth", ASTRALANE_URL))
        .header("api_key", ASTRALANE_API_KEY)
        .send()
        .await?;
    println!("send getHealth to keep connection alive");

    let body = txn_payload_builder(&keypair, recent_blockhash).await?;
    let time = Instant::now();
    let _res = client
        .post(ASTRALANE_URL)
        .header("api_key", ASTRALANE_API_KEY)
        .json(&body)
        .send()
        .await?;
    let elapsed = time.elapsed();
    println!("Total time after second request: {}ms", elapsed.as_millis());

    Ok(())
}

#[tokio::main]
async fn main() {
    send_txn_with_keep_alive().await.unwrap();
}
