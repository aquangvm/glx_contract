pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};

use near_sdk::{serde_json::json, AccountId, env::block_timestamp};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, STORAGE_AMOUNT, call, view, block,
};
use add_node::ContractContract as nodeContract;
use voting::ContractContract as votingContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NODE_BYTES => "./out/manager_node.wasm",
    VOTING_BYTES => "./out/voting.wasm",
}
#[test]
pub fn test_all()  {
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


    // add organization
    call!(
        root,
        node.add_organization(organization.account_id())
    ).assert_success();

    // add node
    call!(
        root,
        node.add_node("node1".account_id())
    ).assert_success();

    // set contract manager node
    call!(
        root,
        voting.set_contract_manager_node(node.account_id())
    ).assert_success();

    // add offer
    call!(
        root,
        voting.add_offer("offer_1".to_string(), block_timestamp(), block_timestamp() + 100000, 50)
    ).assert_success();


    println!("infor offer {:?}" , view!(
        root,
        voting.list_offer()
    ).unwrap_json());

    // update offer

    call!(
        root,
        voting.update_offer(0, "offer_1_update".to_string(), block_timestamp(), block_timestamp() + 100000, 60)
    ).assert_success();

    println!("infor offer update {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());

    // remove offer
    call!(
        root,
        voting.remove_offer(0)
    ).assert_success();

    println!("infor offer before remove {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());


    call!(
        root,
        voting.add_offer("offer_1".to_string(), block_timestamp(), block_timestamp() + 100000, 50)
    ).assert_success();

    println!("infor offer before add offer  {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());


    // active offer

    call!(
        root,
        voting.active_offer(1)
    ).assert_success();  

   println!("infor offer after active offer  {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());

    

    // vote

    call!(
        root,
        voting.vote(1)
    ).assert_success();

    println!("infor offer  after actove vote {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());

    // close vote
    call!(
        root,
        voting.close_vote(1)
    ).assert_success();

    println!("infor offer affter close vote  {:?}", view!(
        root,
        voting.list_offer()
    ).unwrap_json());
  
}





