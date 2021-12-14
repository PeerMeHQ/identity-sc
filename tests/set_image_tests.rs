elrond_wasm::imports!();

use elrond_wasm_debug::{assert_sc_error, managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*};
use identity::{self, Identity};

const SC_WASM_PATH: &'static str = "../output/adder.wasm";
const COST_TOKEN_ID: &[u8] = b"COST-abcdef";

#[test]
fn it_stores_nft_as_image() {
    let mut blockchain = BlockchainStateWrapper::new();
    let caller = blockchain.create_user_account(&rust_biguint!(1_000));
    let contract = blockchain.create_sc_account(&rust_biguint!(0), None, identity::contract_obj, SC_WASM_PATH);

    blockchain.set_esdt_balance(&caller, COST_TOKEN_ID, &rust_biguint!(1_000));
    blockchain.set_esdt_local_roles(&contract.address_ref(), COST_TOKEN_ID, &[EsdtLocalRole::Burn]);

    blockchain.execute_esdt_transfer(&caller, &contract, COST_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
        sc.init(managed_token_id!(COST_TOKEN_ID), managed_biguint!(100));

        let result = sc.set_image(
            managed_token_id!(COST_TOKEN_ID),
            managed_biguint!(100),
            managed_token_id!(b"COL-123456"),
            2,
        );

        assert_eq!(result, SCResult::Ok(()));

        StateChange::Commit
    });

    blockchain.execute_query(&contract, |sc| {
        let actual = sc.image_nfts_by_address(&managed_address!(&caller)).get();
        assert_eq!(managed_token_id!(b"COL-123456"), actual.token_id);
        assert_eq!(2, actual.nonce);
    });
}

#[test]
fn it_burns_cost_tokens() {
    let mut blockchain = BlockchainStateWrapper::new();
    let caller = blockchain.create_user_account(&rust_biguint!(1_000));
    let contract = blockchain.create_sc_account(&rust_biguint!(0), None, identity::contract_obj, SC_WASM_PATH);

    blockchain.set_esdt_balance(&caller, COST_TOKEN_ID, &rust_biguint!(1_000));
    blockchain.set_esdt_local_roles(&contract.address_ref(), COST_TOKEN_ID, &[EsdtLocalRole::Burn]);

    blockchain.execute_esdt_transfer(&caller, &contract, COST_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
        sc.init(managed_token_id!(COST_TOKEN_ID), managed_biguint!(100));

        let result = sc.set_image(
            managed_token_id!(COST_TOKEN_ID),
            managed_biguint!(100),
            managed_token_id!(b"COL-123456"),
            2,
        );

        assert_eq!(result, SCResult::Ok(()));

        StateChange::Commit
    });

    blockchain.execute_query(&contract, |sc| {
        assert_eq!(managed_biguint!(100), sc.burned_tokens().get());
    });

    blockchain.check_esdt_balance(&contract.address_ref(), COST_TOKEN_ID, &rust_biguint!(0));
}

#[test]
fn it_fails_when_paid_with_wrong_cost_token() {
    let mut blockchain = BlockchainStateWrapper::new();
    let caller = blockchain.create_user_account(&rust_biguint!(1_000));
    let contract = blockchain.create_sc_account(&rust_biguint!(0), None, identity::contract_obj, SC_WASM_PATH);

    blockchain.set_esdt_balance(&caller, COST_TOKEN_ID, &rust_biguint!(1_000));

    blockchain.execute_esdt_transfer(&caller, &contract, COST_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
        sc.init(managed_token_id!(COST_TOKEN_ID), managed_biguint!(100));

        let result = sc.set_image(
            managed_token_id!(b"INVALID-abcdef"),
            managed_biguint!(100),
            managed_token_id!(b"TEST-123456"),
            0,
        );

        assert_sc_error!(result, b"invalid token");

        StateChange::Commit
    });
}

#[test]
fn it_fails_when_paid_amount_is_less_than_cost_amount() {
    let mut blockchain = BlockchainStateWrapper::new();
    let caller = blockchain.create_user_account(&rust_biguint!(1_000));
    let contract = blockchain.create_sc_account(&rust_biguint!(0), None, identity::contract_obj, SC_WASM_PATH);

    blockchain.set_esdt_balance(&caller, COST_TOKEN_ID, &rust_biguint!(1_000));

    blockchain.execute_esdt_transfer(&caller, &contract, COST_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
        let cost_token = managed_token_id!(COST_TOKEN_ID);

        sc.init(cost_token.clone(), managed_biguint!(100));

        let result = sc.set_image(cost_token, managed_biguint!(90), managed_token_id!(b"TEST-123456"), 0);

        assert_sc_error!(result, b"invalid amount");

        StateChange::Commit
    });
}
