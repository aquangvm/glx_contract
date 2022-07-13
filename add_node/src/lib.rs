use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, env, PanicOnDefault, Balance, EpochHeight, BlockHeight, BorshStorageKey, Promise, PromiseResult, PromiseOrValue, ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Node {
    pub account_organization: AccountId,
    pub account_node: AccountId,
    pub node_valid: bool,
}

impl Node {
    fn new(account_organization: AccountId,account_node: AccountId, node_valid: bool) -> Self {
        Self {
            account_organization: account_organization,
            account_node: account_node,
            node_valid: node_valid,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    pub list_admin: Vec<AccountId>,
    pub list_organization: Vec<AccountId>,
    #[serde(skip_serializing)]
    pub list_node: UnorderedMap<AccountId, Node>,
    pub pause: bool,
}


impl Contract {
    pub fn intenal_pause_contract(&mut self) {
        self.pause = true;
    }

    pub fn intenal_un_pause_contract(&mut self) {
        self.pause = false;
    }

    pub fn intenal_check_network_admin(&self, account_admin: AccountId) -> bool {
        if self.list_admin.len() > 0 {
            for admin in self.list_admin.clone() {
            assert!(self.pause == false, "contract not ready");
            if admin == account_admin {
                return true;
            }
        };
        }
        
        if self.list_admin.len() == 0 {
             return true;
        }
        false
    }

    pub fn internal_set_key_admin(&mut self, account_admin_new: AccountId) {
        assert!(self.intenal_check_network_admin(env::signer_account_id()) == true, "is not admin");
        self.list_admin.push(account_admin_new);
        if self.pause == true {
            self.pause = false;
        }
    }

    pub fn interanl_remove_key_admin(&mut self,account_remove: AccountId) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.intenal_check_network_admin(env::signer_account_id()) == true, "is not admin");
        for remove_index in 0..self.list_admin.len() {
            if account_remove == *(self.list_admin.get(remove_index).unwrap()) {
                self.list_admin.remove(remove_index);
                break;
            }        
        }
        }

    pub fn internal_add_organization(&mut self,account_organization: AccountId) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.intenal_check_network_admin(env::signer_account_id()) == true, "is not admin");
       
        self.list_organization.push(account_organization)
    }

    pub fn internal_update_organization(&mut self, account_organization_old: AccountId, account_organization_new: AccountId) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.intenal_check_network_admin(env::signer_account_id()) == true, "is not admin");   
        self.list_organization.push(account_organization_new);

       for remove_index in 0..self.list_organization.len() {
            if account_organization_old == *(self.list_organization.get(remove_index).unwrap()) {
                self.list_admin.remove(remove_index);
                break;
            }
            break;
        }
    }

    pub fn internal_remove_organization(&mut self, account_organization: AccountId) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.intenal_check_network_admin(env::signer_account_id()) == true, "is not admin");

        for remove_index in 0..self.list_organization.len() {
            if account_organization == *(self.list_organization.get(remove_index).unwrap()) {
                self.list_organization.remove(remove_index);
                break;
            }
        }
        
        let mut list_node_remove = Vec::new();
        for accountId in self.list_node.keys_as_vector().iter() {

            if self.list_node.get(&accountId.clone()).unwrap().account_organization == account_organization {
                
                list_node_remove.push(accountId)
            }        
        }

        for index in 0..list_node_remove.len() {
            self.list_node.remove(&*(list_node_remove.get(index).unwrap()));
        }
    }

    pub fn internal_check_organization(&self, account_organization: AccountId) -> bool {
        assert!(self.pause == false, "contract not ready");
        for organization  in self.list_organization.clone() {
            if organization.eq(&account_organization) {
                return true;
            }
        }
        false
    }

    pub fn internal_add_node(&mut self, account_node: AccountId, node_vali: bool) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.internal_check_organization(env::signer_account_id()) == true, "is not organization");
      
        let node = Node::new(env::signer_account_id(),account_node.clone(), node_vali);
        self.list_node.insert(&account_node, &node);

        // let mut list_node = self.list_organization.get(&key_organization).unwrap().list_node;
        // list_node.push(key_node);
    }

    pub fn internal_remove_node(&mut self, account_node: AccountId, node_vali: bool) {
        assert!(self.pause == false, "contract not ready");
        assert!(self.internal_check_organization(env::signer_account_id()) == true, "is not organization");

        if self.list_node.get(&account_node).unwrap().account_organization == env::signer_account_id() {
            self.list_node.remove(&account_node);
        }
        
    }


    pub fn internal_check_node(&self, key_node: AccountId) -> bool{
        assert!(self.pause == false, "contract not ready");
        match self.list_node.get(&key_node) {
            Some (a) => true,
            None => false,
        } 
    }

    pub fn internal_check_node_vali(&self, key_node: AccountId) -> bool {
        assert!(self.pause == false, "contract not ready");
        //self.list_node.get(&key_node).unwrap().node_valid
        let mut node_validate = false;
        match self.list_node.get(&key_node) {
            Some (a) =>  node_validate = a.node_valid ,
            None =>  env::panic(b"not found"),
        }; 
        node_validate
    }

    pub fn internal_count_organization(&self) -> u64 {
        self.list_organization.len() as u64
    }

   
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
       // let list_organization = UnorderedMap::new("new list organization".as_bytes());
        let list_node = UnorderedMap::new("new list node".as_bytes());

        Self { list_admin: Vec::new(), list_organization: Vec::new(),list_node: list_node, pause: true }  
    }

    pub fn pause_contract(&mut self) {
        self.intenal_pause_contract();
    }

    pub fn un_pause_contract(&mut self) {
        self.intenal_un_pause_contract();
    }

    pub fn set_key_admin(&mut self, key_admin: AccountId) {
        self.internal_set_key_admin(key_admin);
    }

    pub fn remove_key_admin(&mut self, key_remove: AccountId) {
        self.interanl_remove_key_admin(key_remove);
    }

    pub fn check_network_admin(&self, key_admin: AccountId) -> bool {
        self.intenal_check_network_admin(key_admin)
    }

    pub fn add_organization(&mut self, key_organization: AccountId) {
        self.internal_add_organization(key_organization);
    }

    pub fn update_organization_admin(
        &mut self,
        key_organization_old: AccountId,
        key_organization_new: AccountId,
    ) {
        self.internal_update_organization( key_organization_old, key_organization_new);
    }

    pub fn remove_organization(&mut self, key_organization: AccountId) {
        self.internal_remove_organization(key_organization);
    }

    pub fn add_node(&mut self, key_node: AccountId ) {
        self.internal_add_node(key_node, false);
    }

    pub fn remove_node(&mut self, key_node: AccountId, node_vali: bool) {
        self.internal_remove_node( key_node, node_vali);
    }

    pub fn check_node(&self, key_node: AccountId) -> bool{
        self.internal_check_node(key_node)
    }

    pub fn check_node_vali(&self, key_node: AccountId) -> bool {
        self.internal_check_node_vali(key_node)
    }

    pub fn check_organization(&self, key_organization: AccountId) -> bool {
        self.internal_check_organization(key_organization)
    }

    pub fn count_organization(&self) -> u64 {
        self.internal_count_organization()
    }

    // pub fn update_node(&mut self, account_node: AccountId) {
    //     self.list_node.get(&account_node).unwrap().node_valid = true;
    // }

    
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain};

    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    fn get_context_admin() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("admin.near"));
        builder.current_account_id(to_valid_account("admin.near"));
        builder.signer_account_id(to_valid_account("admin.near"));
        builder
    }

    fn get_context_admin1() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("admin1.near"));
        builder.current_account_id(to_valid_account("admin1.near"));
        builder.signer_account_id(to_valid_account("admin1.near"));
        builder
    }

    fn get_context_organization() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("organization.near"));
        builder.current_account_id(to_valid_account("organization.near"));
        builder.signer_account_id(to_valid_account("organization.near"));
        builder
    }

    fn get_context_organization1() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("organization1.near"));
        builder.current_account_id(to_valid_account("organization1.near"));
        builder.signer_account_id(to_valid_account("organization1.near"));
        builder
    }

    fn get_context_node() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(to_valid_account("node.near"));
        builder.current_account_id(to_valid_account("node.near"));
        builder.signer_account_id(to_valid_account("node.near"));
        builder
    }

    #[test]
    fn test_all() {
        let mut context = get_context_admin();
        testing_env!(context.build());


        let mut contract = Contract::new();

        contract.set_key_admin(to_valid_account("admin.near"));

        context.signer_account_id(to_valid_account("admin.near"));
        contract.set_key_admin(to_valid_account("admin1.near"));


        assert_eq!(true, contract.check_network_admin(to_valid_account("admin.near")));

        println!("check admin {}", contract.check_network_admin(to_valid_account("admin.near")));

        assert_eq!(true, contract.check_network_admin(to_valid_account("admin1.near")));
        

        context.signer_account_id(to_valid_account("admin.near"));
        contract.remove_key_admin(to_valid_account("admin1.near"));



        println!("check admin1  {}", contract.check_network_admin(to_valid_account("admin1.near")));

        assert_eq!(false, contract.check_network_admin(to_valid_account("admin1.near")));

        context.signer_account_id(to_valid_account("admin.near"));
        contract.add_organization(to_valid_account("organization.near"));
        println!("check organization  {}", contract.check_organization(to_valid_account("organization.near")));
        assert_eq!(true, contract.check_organization(to_valid_account("organization.near")));

        contract.add_organization(to_valid_account("organization1.near"));
        assert_eq!(true, contract.check_organization(to_valid_account("organization1.near")));
        contract.remove_organization(to_valid_account("organization1.near"));

        println!("check organization again {}", contract.check_organization(to_valid_account("organization1.near")));
        assert_eq!(false, contract.check_organization(to_valid_account("organization1.near")));


        let mut context1 = get_context_organization();
        testing_env!(context1.build());
       
        context1.signer_account_id(to_valid_account("organization.near"));
        contract.add_node(to_valid_account("node.near"));


        assert_eq!(true, contract.check_node(to_valid_account("node.near")));

        assert_eq!(false, contract.check_node_vali(to_valid_account("node.near")));
        println!("so organization :  {}", contract.count_organization());

      //  contract.update_node(to_valid_account("node.near"));

    //    // println!("aaa {}", contract.check_node_vali(to_valid_account("node.near")));

    //     assert_eq!(true, contract.check_node_vali(to_valid_account("node.near")));

        
 

    }

}

