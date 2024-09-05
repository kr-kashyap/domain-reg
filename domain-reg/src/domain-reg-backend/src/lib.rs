
use ic_ledger_types::{Subaccount, Tokens};
use ic_cdk::{println, caller};
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
    let mut pp = Some(Tokens::from_e8s(0));
    let mut tp = Some(Tokens::from_e8s(0));

    STATE.with(|state| {
        let state = state.borrow();
        pp = if state.purchase_price.is_some() {
            Some(state.purchase_price.unwrap())
        } else { pp };
        tp = if state.transfer_price.is_some() {
            Some(state.transfer_price.unwrap())
        } else { tp };
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
    let mut owner = Principal::anonymous();
    STATE.with(|state| {
        let state = state.borrow();

        let record_ = &state.records;
        let new_owner = record_.as_ref()
        .and_then(|vec| vec.into_iter().find(|record| record.registered_name == arg).map(|record| record.owner));

        // Handle the Option<Principal>
        match new_owner {
            Some(own) => {
                println!("Found {}",own);
                owner = own
            },
            None => println!("No matching record found")
        }
    });

    let resolve_record_response = ResolveRecordResponse {
        address : if owner != Principal::anonymous() {
            Some(owner.to_string())
        } else { Some(String::from("")) }
    };
    resolve_record_response
}

#[update]
#[candid::candid_method]
fn register(domain_name : String) -> RegisterReceipt {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        push_to_option_vec(&mut state.records, Record { owner : caller(), registered_name : domain_name});
    });
    let resolve_record_response = ResolveRecordResponse {
        address: Some(caller().to_string()),
    };
    RegisterReceipt::Ok(resolve_record_response)
}

// Function to push a new record into Option<Vec<Record>>
fn push_to_option_vec(records: &mut Option<Vec<Record>>, new_record: Record) {
    match records {
        // If the Option is Some, push the new record to the existing Vec
        Some(vec) => vec.push(new_record),
        // If the Option is None, create a new Vec with the new record
        None => *records = Some(vec![new_record]),
    }
}

#[update]
#[candid::candid_method]
fn transfer(domain_name: String, new_owner : Principal) -> TransferReceipt {
    let resolve_record_response = ResolveRecordResponse {
        address: Some(caller().to_string()),
    };
    TransferReceipt::Ok(resolve_record_response)
}

#[derive(CandidType, Deserialize)]
struct InitArgs {
    purchase_price: Option<Tokens>,
    transfer_price: Option<Tokens>,
    // treasury_account: Account,
    records : Option<Vec<Record>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

#[init]
fn init(args: InitArgs) {
    ic_cdk::setup();

    println!("arg.purchase_price : {:?}",args.purchase_price);

    let pp = Some(Tokens::from_e8s(0));
    let tp = Some(Tokens::from_e8s(0));

    STATE.with(|state| {
        let default_record = Some(vec![Record { owner : Principal::anonymous(), registered_name : String::from("")}]);
        let mut state = state.borrow_mut();
        state.purchase_price = if args.purchase_price.is_some() {
            Some(args.purchase_price.unwrap())
        } else { pp };
        state.transfer_price = if args.transfer_price.is_some() {
            Some(args.transfer_price.unwrap())
        } else { println!("state.record"); tp };
        // state.treasury_account = args.treasury_account;
        state.records = if args.records.is_some() {
            Some(args.records.unwrap())
        } else {  default_record };
        println!("state.purchase_price : {:?}",state.purchase_price);
    });
}

impl Default for InitArgs {
    fn default() -> Self {
        let default_record = Some(vec![Record { owner : Principal::anonymous(), registered_name : String::from("abc")}]);
        InitArgs {
            purchase_price: Some(Tokens::from_e8s(0)),
            transfer_price: Some(Tokens::from_e8s(0)),
            records : default_record,
        }
        // treasury_account: Account {
        //     owner: Principal::anonymous(), 
        //     subaccount: None,
        // },
    }
}