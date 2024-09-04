
use ic_ledger_types::Subaccount;

use candid::{CandidType, Principal};

use std::cell::RefCell;
use serde::{Deserialize, Serialize};

mod types;
use types::*;

use ic_cdk_macros::*;

thread_local! {
    static STATE: RefCell<InitArgs> = RefCell::default();
}

#[query]
#[candid::candid_method(query)]
fn config() -> ConfigResponse {
    let mut pp = Some(0);
    let mut tp = Some(0);

    STATE.with(|state| {
        let state = state.borrow();
        pp = state.purchase_price;
        tp = state.transfer_price;
    });
    let config_response = ConfigResponse {
        purchase_price: pp,
        transfer_price: tp,
    };
    config_response
}

#[query]
#[candid::candid_method(query)]
fn resolve_record(arg: String) -> ResolveRecordResponse {
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(arg+" abc")),
    };
    resolve_record_response
}

#[update]
#[candid::candid_method]
fn register(domain_name : String) -> RegisterReceipt {
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(domain_name+" abc")),
    };
    RegisterReceipt::Ok(resolve_record_response)
}

#[update]
#[candid::candid_method]
fn transfer(domain_name: String, new_name : String) -> TransferReceipt {
    let s = Box::leak(new_name.into_boxed_str());
    let s_slice: &str = &s[..]; 
    let output = domain_name + " " + s_slice + " abc";
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(output)),
    };
    TransferReceipt::Ok(resolve_record_response)
}

#[derive(CandidType, Deserialize)]
struct InitArgs {
    purchase_price: Option<u64>,
    transfer_price: Option<u64>,
    // treasury_account: Account,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

#[init]
fn init(args: InitArgs) {
    ic_cdk::setup();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.purchase_price = args.purchase_price;
        state.transfer_price = args.transfer_price;
        // state.treasury_account = args.treasury_account;
    });
}

impl Default for InitArgs {
    fn default() -> Self {
        InitArgs {
            purchase_price: Some(0),
            transfer_price: Some(0),
        }
        // treasury_account: Account {
        //     owner: Principal::anonymous(), 
        //     subaccount: None,
        // },
    }
}