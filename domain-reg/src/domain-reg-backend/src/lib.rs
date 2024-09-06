
use ic_ledger_types::{Subaccount, Tokens};
use ic_cdk::{println, caller};
use candid::{CandidType, Principal};

use std::{borrow::Borrow, cell::RefCell};
use std::collections::HashSet;
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
fn push_to_option_vec(records: &mut Option<HashSet<Record>>, new_record: Record) {
    // match records {
    //     // If the Option is Some, push the new record to the existing Vec
    //     Some(vec) => vec.insert(new_record),
    //     // If the Option is None, create a new Vec with the new record
    //     None => println!("")
    //     // {
    //     //     println!("None"),
    //     //     let mut new_set = HashSet::new();
    //     //     new_set.insert(new_record);
    //     //     *records = Some(new_set);
    //     // }
    // }

    // Insert the new record into the HashSet inside the Option if it exists
    if let Some(ref mut set) = records {
        set.insert(new_record);
        println!("Record inserted successfully.");
    } else {
        let mut new_set = HashSet::new();
        new_set.insert(new_record);
        *records = Some(new_set);
        println!("No HashSet found inside the Option.");
    }
}

#[update]
#[candid::candid_method]
fn transfer(domain_name: String, new_owner : Principal) -> TransferReceipt {
    let mut err = TransferErr::None;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        println!("{:?}", state.records);
        // Find the record, check the owner, and update if conditions are met
        if let Some(ref mut set) = state.records {
        // Find the record that matches the name and clone it
            if let Some(record) = set.iter().find(|record| record.registered_name == domain_name).cloned() {
                // Check if the current owner matches the expected owner
                if record.owner == caller() {
                    // Remove the existing record
                    set.take(&record);
                    // Update the owner field
                    let mut updated_record = record;
                    updated_record.owner = new_owner;
                    // Reinsert the updated record back into the HashSet
                    set.insert(updated_record);
                    println!("Record updated successfully.");
                } else {
                    err = TransferErr::NotAllowed;
                    println!("Current owner does not match; transfer not performed.");
                }
            } else {
                println!("Record not found.");
            }
        }
        println!("{:?}", state.records);
    });
    // Print the updated Option<HashSet<Record, RandomState>>
    let resolve_record_response = ResolveRecordResponse {
        address: Some(new_owner.to_string()),
    };
    match err {
        TransferErr::None =>  TransferReceipt::Ok(resolve_record_response),
        TransferErr::NotAllowed => TransferReceipt::Err(TransferErr::NotAllowed),
        TransferErr::InsufficientTokens => TransferReceipt::Err(TransferErr::InsufficientTokens),
        TransferErr::NotExistingDomain => TransferReceipt::Err(TransferErr::NotExistingDomain)
    }
}

#[derive(CandidType, Deserialize)]
struct InitArgs {
    purchase_price: Option<Tokens>,
    transfer_price: Option<Tokens>,
    // treasury_account: Account,
    records : Option<HashSet<Record>>,
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
    } else {  None };
        println!("state.purchase_price : {:?}",state.purchase_price);
    });
}

impl Default for InitArgs {
    fn default() -> Self {
        InitArgs {
            purchase_price: Some(Tokens::from_e8s(0)),
            transfer_price: Some(Tokens::from_e8s(0)),
            records : Some(HashSet::new()),
        }
        // treasury_account: Account {
        //     owner: Principal::anonymous(), 
        //     subaccount: None,
        // },
    }
}