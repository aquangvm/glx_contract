pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};

use near_sdk::{serde_json::json, AccountId};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, STORAGE_AMOUNT, call, view,
};
use add_node::ContractContract as nodeContract;
use voting::ContractContract as votingContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NODE_BYTES => "./out/manager_node.wasm",
    VOTING_BYTES => "./out/voting.wasm",
}

pub fn init() -> (UserAccount,UserAccount,UserAccount,UserAccount,UserAccount) {
  let root = init_simulator(Node);
  
  let admin = root.create_user("admin".to_string(),to_yocto("100"));
  let organization = root.create_user("organization".to_string(),to_yocto("100"));


  // deploy contract voting
  let voting : ContractAccount<votingContract> = deploy! {
    contract: votingContract,
    contract_id: "voting".to_string(),
    bytes: &VOTING_BYTES,
    signer_account: root
  };

  call!(
    root,
    voting.new()
  ).assert_success();

  // deploy contract manager node
  let node : ContractAccount<nodeContract> = deploy! {
    contract: nodeContract,
    contract_id: "node".to_string(),
    bytes: &NODE_BYTES,
    signer_account: root
  };

  call!(
    root,
    node.new()
  ).assert_success();
  (root, voting, node,admin,organization)
}


#[test]
fn contract_manager_node() {
    let (root, voting, node, admin, organization) = init();

    // set key admin
    call!(
        root,
        node.set_key_admin(admin.account_id())
    ).assert_success();

    
    let check_admin: bool = view!(
        root,
        node.check_network_admin(admin.account_id())
    ).unwrap_json();

    println!("network admin:  {}", check_admin);

    assert_eq!(true, check_admin);


    // // add organization
    // call!(
    //     root,
    //     node.
    // )

}

