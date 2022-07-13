use std::str::FromStr;
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, env, PanicOnDefault, Balance, EpochHeight, BlockHeight, BorshStorageKey, Promise, PromiseResult, PromiseOrValue, ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::env::block_timestamp;

// pub const CALLBACK_GAS: Gas = Gas(5_000_000_000_000);
#[derive(Debug)]
#[derive(BorshDeserialize, BorshSerialize, Serialize,PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    PENDING,
    ACTIVE,
    COMPLETE,
    FALSE,
}

#[derive(Debug)]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    offer_id: u64,
    offer: String,
    time_start: u64,
    time_end : u64,
    total_vote: u64,
    target: u64,
    status: Status,
}


#[ext_contract(ext_contract)]
trait ExtContract {
    fn count_organization(&self) -> u64;
        
}

#[ext_contract(ext_callback)]
trait CallbackContract {
    fn my_callback(&self) -> u64;
}

impl Offer {
    fn new(offer_id: u64,offer: String, time_start: u64, time_end: u64, target: u64) -> Self {
        Self {offer_id: offer_id, offer: offer, time_start: time_start, time_end: time_end,total_vote : 0, target: target, status: Status::PENDING }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    #[serde(skip_serializing)]
    list_offer: UnorderedMap<u64,Offer>,
    total_offer: u64,
    contract_manager_node : AccountId,
    total_organization: u64,
}


impl Contract {
    
    pub fn internal_add_offer(&mut self,offer: String, time_start: u64, time_end: u64, target: u64) {
        let offer = Offer::new(self.total_offer,offer, time_start, time_end, target);
        
        self.list_offer.insert(&self.total_offer,&offer);
        self.total_offer = self.total_offer + 1;
    }

    pub fn internal_list_offer(&self) -> Vec<Offer> {
        let mut list_offer = Vec::new();
        for offer in self.list_offer.values_as_vector().iter() {
            list_offer.push(offer);
        };

        list_offer
    }

    pub fn internal_view_offer(&self, offer_id: u64) -> Offer {

        match self.list_offer.get(&offer_id) {
            Some(a) => a,
            None => env::panic(b"not found"),
        }
    }

    pub fn internal_update_offer(&mut self, offer_id: u64, offer: String, time_start: u64, time_end: u64, target: u64) {
        match self.list_offer.get(&offer_id.clone()) {
            Some(a) => assert!(a.status == Status::PENDING, "offer is active"),
            None => env::panic(b"not found"),
        } 
        let offer = Offer::new(offer_id, offer, time_start, time_end, target);

        self.list_offer.insert(&offer_id, &offer);
    }

    pub fn internal_remove_offer(&mut self, offer_id: u64) {
        match self.list_offer.get(&offer_id.clone()) {
            Some(a) => assert!(a.status == Status::PENDING, "offer is active"),
            None => env::panic(b"not found"),
        }

        self.list_offer.remove(&offer_id);
    }

    pub fn internal_active_offer(&mut self, offer_id: u64) {
        match self.list_offer.get(&offer_id.clone()) {
            Some(a) => assert!(a.status == Status::PENDING, "offer is active"),
            None => env::panic(b"not found"),
        }

        //self.list_offer.get(&offer_id).unwrap().status = Status::ACTIVE;
        let mut offer = self.list_offer.get(&offer_id).unwrap();
        offer.status = Status::ACTIVE;

        self.list_offer.insert(&offer_id, &offer);
       // self.list_offer.get(&offer_id).unwrap().status
        // let mut status1 = self.list_offer.get(&offer_id).unwrap().status;
        // status1 = Status::ACTIVE;


       // assert!(self.list_offer.get(&offer_id.clone()).unwrap().status == Status::ACTIVE, "sai roi");

    }

    pub fn internal_vote(&mut self, offer_id: u64) {
        match self.list_offer.get(&offer_id.clone()) {
            Some(a) => assert!(a.status == Status::ACTIVE, "offer is active"),
            None => env::panic(b"not found"),
        }

        assert!(block_timestamp() >= self.list_offer.get(&offer_id.clone()).unwrap().time_start, "time out");
        assert!(block_timestamp() <= self.list_offer.get(&offer_id.clone()).unwrap().time_end, "time out");
        
        self.list_offer.get(&offer_id.clone()).unwrap().total_vote = self.list_offer.get(&offer_id.clone()).unwrap().total_vote + 1;
    }

    pub fn internal_set_contract_manager_node (&mut self, account_contract: AccountId) {
        self.contract_manager_node = account_contract;
    }

}

#[near_bindgen]
impl Contract {
    pub fn new() -> Self {
        let list_offer = UnorderedMap::new("new list offer".as_bytes());
        Self { 
            list_offer: list_offer,
            total_offer: 0, 
            contract_manager_node: env::current_account_id(),
            total_organization: 0 
            }
    }

    pub fn add_offer(&mut self,offer: String, time_start: u64, time_end: u64, target: u64) {
        self.internal_add_offer(offer, time_start, time_end, target);
    }

    pub fn update_offer(&mut self, offer_id: u64, offer: String, time_start: u64, time_end: u64, target: u64) {
        self.internal_update_offer(offer_id, offer, time_start, time_end, target);
      // self.list_offer.get(&offer_id).unwrap().offer = "quang".to_string();
    }

    pub fn remove_offer(&mut self, offer_id: u64) {
        self.internal_remove_offer(offer_id);
    }

    pub fn active_offer(&mut self, offer_id: u64) {
        self.internal_active_offer(offer_id);
    }

    pub fn list_offer(&self) -> Vec<Offer> {
        self.internal_list_offer()
    }

    pub fn view_offer(&self, offer_id: u64) -> Offer {
        self.internal_view_offer(offer_id)
    }

    pub fn vote(&mut self, offer_id: u64) {
        self.internal_vote(offer_id);
    }

    #[payable]
    pub fn close_vote(&mut self, offer_id : u64) {
        match self.list_offer.get(&offer_id.clone()) {
            Some(a) => assert!(a.status == Status::ACTIVE, "offer is active"),
            None => env::panic(b"not found"),
        }
        assert!(block_timestamp() >= self.list_offer.get(&offer_id.clone()).unwrap().time_end, "time out");

        // ext_contract::ext(AccountId::from_str(
        //     self.contract_manager_node.as_str())
        //     .unwrap())
        //     .with_attached_deposit(0)
        //     .with_static_gas(CALLBACK_GAS).
        //     count_organization()
        //     .then(ext_callback::ext(env::current_account_id()).my_callback());
        ext_contract::count_organization(&AccountId::from_str(self.contract_manager_node.as_str()).unwrap(), 0, 5_000_000_000_000)
        .then(ext_callback::my_callback(&env::current_account_id(), 0, 5_000_000_000_000));
        let mut check_target = (self.list_offer.get(&offer_id.clone()).unwrap().total_vote * 100 ) / self.total_organization;

        if check_target >= self.list_offer.get(&offer_id.clone()).unwrap().target {
            self.list_offer.get(&offer_id).unwrap().status = Status::COMPLETE;
        }
        else {
            self.list_offer.get(&offer_id).unwrap().status = Status::FALSE;
        }
        
    }

    #[private]
    pub fn my_callback(&mut self) {
     match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_value) => {
                match String::from_utf8(_value).unwrap_or("0".to_string()).as_str().parse::<u64>() {
                    Ok(a) => {
                        self.total_organization = a;
                    },
                    _ => env::panic(b"ERR_CALL_FAILED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain};

    fn to_valid_account(account: &str) -> AccountId {
        AccountId::try_from(account.to_string()).expect("Invalid account")
    }

    fn get_context_admin() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("admin.near"));
        builder.current_account_id(to_valid_account("admin.near"));
        builder.signer_account_id(to_valid_account("admin.near"));
        builder
    }


    #[test]
    fn test() {
        let mut context = get_context_admin();
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.add_offer("aaaa".to_string(), block_timestamp(), block_timestamp(), 50);

        contract.update_offer(0, "bbbb".to_string(), block_timestamp(), block_timestamp(), 70);

        let listOffer = contract.view_offer(0);

        
            println!("update offfer :  {:?}" , listOffer);

        println!("list offer before remove offer:  {:?}", contract.list_offer());


        contract.remove_offer(0);

       
        contract.add_offer("aaaa".to_string(), block_timestamp(), block_timestamp(), 50);

        println!("lis offer after remove offer:  {:?}", contract.list_offer());


        contract.active_offer(1);

    
        // println!("lis offer :  {:?}", contract.list_offer());
        println!("lis offer after vote:  {:?}", contract.list_offer());

        contract.vote(1);


        println!("lis offer after vote:  {:?}", contract.list_offer());
        
    }
}

