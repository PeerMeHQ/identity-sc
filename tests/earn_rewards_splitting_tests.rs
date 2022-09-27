elrond_wasm::imports!();

use elrond_wasm_debug::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_splits_funds_proportionally_among_stakers() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_one = setup.blockchain.create_user_account(&rust_biguint!(1));
    let user_two = setup.blockchain.create_user_account(&rust_biguint!(1));
    let user_three = setup.blockchain.create_user_account(&rust_biguint!(1));

    setup.blockchain.set_esdt_balance(&user_one, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(100));
    setup.blockchain.set_esdt_balance(&user_two, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(100));
    setup.blockchain.set_esdt_balance(&user_three, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(100));

    // user one stakes – 25%
    setup
        .blockchain
        .execute_esdt_transfer(&user_one, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(25), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // user one stakes again – 25%
    setup
        .blockchain
        .execute_esdt_transfer(&user_one, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(25), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // user two stakes – 30%
    setup
        .blockchain
        .execute_esdt_transfer(&user_two, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(30), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // user three stakes – 20%
    setup
        .blockchain
        .execute_esdt_transfer(&user_three, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(20), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // distribute reward tokens
    setup
        .blockchain
        .execute_esdt_transfer(&setup.owner_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.distribute_to_earn_core_endpoint();
        })
        .assert_ok();

    // assert balances
    setup
        .blockchain
        .execute_query(&setup.contract, |sc| {
            assert_eq!(sc.compute_reward(&managed_address!(&user_one)), managed_biguint!(50));
            assert_eq!(sc.compute_reward(&managed_address!(&user_two)), managed_biguint!(30));
            assert_eq!(sc.compute_reward(&managed_address!(&user_three)), managed_biguint!(20));
        })
        .assert_ok();
}

#[test]
fn it_splits_funds_proportionally_among_stakers_that_withdraw_in_between() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_one = setup.blockchain.create_user_account(&rust_biguint!(1));
    let user_two = setup.blockchain.create_user_account(&rust_biguint!(1));

    setup.blockchain.set_esdt_balance(&user_one, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(100));
    setup.blockchain.set_esdt_balance(&user_two, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(100));

    // user one stakes – 50%
    setup
        .blockchain
        .execute_esdt_transfer(&user_one, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(50), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // user two stakes – 50%
    setup
        .blockchain
        .execute_esdt_transfer(&user_two, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(50), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    // distribute reward tokens
    setup
        .blockchain
        .execute_esdt_transfer(&setup.owner_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.distribute_to_earn_core_endpoint();

            assert_eq!(sc.compute_reward(&managed_address!(&user_one)), managed_biguint!(50));
            assert_eq!(sc.compute_reward(&managed_address!(&user_two)), managed_biguint!(50));
        })
        .assert_ok();

    setup.blockchain.set_block_timestamp(EARN_STAKE_LOCK_TIME_SECONDS + 1);

    // user two withdraws – 50% of initial stake
    setup
        .blockchain
        .execute_tx(&user_two, &setup.contract, &rust_biguint!(0), |sc| {
            sc.withdraw_from_earn_endpoint(managed_token_id!(EARN_STAKE_CORE_TOKEN_ID), managed_biguint!(25));
        })
        .assert_ok();

    // distribute again
    setup
        .blockchain
        .execute_esdt_transfer(&setup.owner_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.distribute_to_earn_core_endpoint();

            assert_eq!(sc.compute_reward(&managed_address!(&user_one)), managed_biguint!(116));
            assert_eq!(sc.compute_reward(&managed_address!(&user_two)), managed_biguint!(83));
        })
        .assert_ok();
}
