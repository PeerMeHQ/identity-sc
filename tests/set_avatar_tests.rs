multiversx_sc::imports!();

use multiversx_sc_scenario::*;
use identity::config::*;
use identity::*;
use setup::*;

mod setup;

#[test]
fn it_stores_an_avatar() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, CORE_TOKEN_ID, &rust_biguint!(1_000));

    setup
        .blockchain
        .execute_esdt_transfer(&setup.user_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.set_avatar_endpoint(managed_token_id!(b"COL-123456"), 2);
        })
        .assert_ok();

    setup
        .blockchain
        .execute_query(&setup.contract, |sc| {
            let actual = sc.avatars(&managed_address!(&user_address)).get();
            assert_eq!(managed_token_id!(b"COL-123456"), actual.token_id);
            assert_eq!(2, actual.nonce);
        })
        .assert_ok();
}

#[test]
fn it_fails_when_paid_with_wrong_cost_token() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, CORE_TOKEN_ID, &rust_biguint!(1_000));

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.set_avatar_endpoint(managed_token_id!(b"TEST-123456"), 0);
        })
        .assert_ok();
}

#[test]
fn it_fails_when_paid_amount_is_less_than_cost_amount() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, CORE_TOKEN_ID, &rust_biguint!(1_000));

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(25), |sc| {
            sc.cost_avatar_set().set(managed_biguint!(200));

            sc.set_avatar_endpoint(managed_token_id!(b"TEST-123456"), 0);
        })
        .assert_user_error("invalid amount");
}
