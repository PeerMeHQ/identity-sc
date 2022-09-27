elrond_wasm::imports!();

use elrond_wasm_debug::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_withdraws_core_stake_tokens() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup.blockchain.set_block_timestamp(EARN_STAKE_LOCK_TIME_SECONDS + 1);

    setup
        .blockchain
        .execute_tx(&user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.withdraw_from_earn_endpoint(managed_token_id!(EARN_STAKE_CORE_TOKEN_ID), managed_biguint!(500));

            assert_eq!(sc.core_stake(&managed_address!(&user_address)).get(), managed_biguint!(0));
            assert_eq!(sc.core_stake_total().get(), managed_biguint!(0));
        })
        .assert_ok();
}

#[test]
fn it_fails_to_withdraw_locked_stake() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup.blockchain.set_block_timestamp(5);

    setup
        .blockchain
        .execute_tx(&user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.withdraw_from_earn_endpoint(managed_token_id!(EARN_STAKE_CORE_TOKEN_ID), managed_biguint!(500));
        })
        .assert_user_error("stake is locked");
}
