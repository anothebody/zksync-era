mod aggregated_operations;
mod aggregator;
mod error;
mod eth_tx_aggregator;
mod eth_tx_manager;
mod metrics;
mod publish_criterion;
mod utils;
mod zksync_functions;

mod abstract_l1_interface;

mod eth_fees_oracle;
#[cfg(test)]
mod tests;

pub use self::{
    aggregator::Aggregator, error::EthSenderError, eth_tx_aggregator::EthTxAggregator,
    eth_tx_manager::EthTxManager,
};

async fn aa() {
    use cosmrs::AccountId;
    use fiamma_sdk_rs::{
        tx::TxClient,
        types::{MsgCreateStaker, MsgRemoveStaker, MsgSubmitProof},
        wallet::Wallet,
    };

    const BITVM_PROOF_SYSTEM: &str = "GROTH16_BN254_BITVM";
    const NAMESPACE: &str = "ZKSYNC-Stack";
    const TEST_DATA: &str = "test-data";
    const SENDER_PRIVATE_KEY: &str =
        "59514b4e9c63b91cc9d3b6b882f1c5ee7449890c7c1116782670c71c96957897";
    const NODE: &str = "http://54.65.137.66:9090";

    let wallet = Wallet::new(SENDER_PRIVATE_KEY);
    let gas_limit = 80_000_000_u64;
    let fee = 2000_u128;
    let tx_client = TxClient::new(SENDER_PRIVATE_KEY, NODE, fee, gas_limit);
    // load proof, vk, public_input
    let (proof, public_input, vk) = proof_artifacts();

    let submit_proof_msg = MsgSubmitProof {
        creator: wallet.account_id,
        proof_system: BITVM_PROOF_SYSTEM.to_string(),
        proof,
        public_input,
        vk,
        namespace: NAMESPACE.to_string(),
    };

    let resp = tx_client.submit_proof(submit_proof_msg).await.unwrap();
    println!("submit_proof resp: {:?}", resp);

    fn proof_artifacts() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let location = std::env::current_dir().unwrap().join(TEST_DATA);

        let proof_file = location.join("proof.bitvm");
        let proof = std::fs::read(&proof_file).unwrap();

        let public_input_file = location.join("public_input.bitvm");
        let public_input = std::fs::read(&public_input_file).unwrap();

        let vk_file = location.join("vk.bitvm");
        let vk = std::fs::read(&vk_file).unwrap();
        (proof, public_input, vk)
    }
}
