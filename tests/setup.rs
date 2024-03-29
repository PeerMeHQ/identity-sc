multiversx_sc::imports!();

use identity::config::*;
use identity::*;
use multiversx_sc_scenario::testing_framework::{BlockchainStateWrapper, ContractObjWrapper};
use multiversx_sc_scenario::*;

pub const CORE_TOKEN_ID: &[u8] = b"SUPER-abcdef";

pub const WASM_PATH: &'static str = "output/identity.wasm";

#[allow(dead_code)]
pub struct ContractSetup<ObjBuilder>
where
    ObjBuilder: 'static + Copy + Fn() -> identity::ContractObj<DebugApi>,
{
    pub blockchain: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract: ContractObjWrapper<identity::ContractObj<DebugApi>, ObjBuilder>,
}

pub fn setup_contract<ObjBuilder>(builder: ObjBuilder) -> ContractSetup<ObjBuilder>
where
    ObjBuilder: 'static + Copy + Fn() -> identity::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain = BlockchainStateWrapper::new();
    let owner_address = blockchain.create_user_account(&rust_zero);
    let user_address = blockchain.create_user_account(&rust_zero);
    let contract = blockchain.create_sc_account(&rust_zero, Some(&owner_address), builder, WASM_PATH);

    blockchain
        .execute_tx(&owner_address, &contract, &rust_zero, |sc| {
            sc.init(managed_token_id!(CORE_TOKEN_ID), managed_biguint!(100));
        })
        .assert_ok();

    ContractSetup {
        blockchain,
        owner_address,
        user_address,
        contract,
    }
}

#[test]
fn it_initializes_the_contract() {
    let mut setup = setup_contract(identity::contract_obj);

    setup
        .blockchain
        .execute_query(&setup.contract, |sc| {
            assert_eq!(managed_token_id!(CORE_TOKEN_ID), sc.core_token().get());
        })
        .assert_ok();
}
