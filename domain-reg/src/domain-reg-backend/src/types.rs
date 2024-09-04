use candid::CandidType;
use ic_ledger_types::Tokens;

#[allow(dead_code)]
#[derive(CandidType)]
pub struct ConfigResponse {
    pub purchase_price: Option<Tokens>,
    pub transfer_price: Option<Tokens>,
}

// We define a custom struct for each query response
#[derive(CandidType)]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}

#[derive(CandidType)]
pub struct Register {
    name: String,
    coin: Tokens,
}

#[derive(CandidType)]
pub struct Transfer {
    name: String,
    to: String,
    coin: Tokens,
}

pub type RegisterReceipt = Result<ResolveRecordResponse, RegisterErr>;

#[derive(CandidType)]
pub enum RegisterErr {
    NotAllowed,
    InsufficientTokens,
}

pub type TransferReceipt = Result<ResolveRecordResponse, TransferErr>;

#[derive(CandidType)]
pub enum TransferErr {
    NotAllowed,
    NotExistingDomain,
    InsufficientTokens,
}

ic_cdk::export_candid!();