multiversx_sc::imports!();

use multiversx_sc_scenario::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_claims_rewards() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();
    let rewards = 500;
    let initial_balance = setup.blockchain.get_esdt_balance(&user_address, CORE_TOKEN_ID, 0);
    let expected_balance = initial_balance + rust_biguint!(rewards);

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_esdt_transfer(&setup.owner_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(rewards), |sc| {
            sc.distribute_to_earn_core_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_tx(&user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.claim_earn_reward_endpoint();
        })
        .assert_ok();

    setup.blockchain.check_esdt_balance(&user_address, CORE_TOKEN_ID, &expected_balance)
}

#[test]
fn it_fails_if_no_rewards_to_claim() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(100), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_tx(&user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.claim_earn_reward_endpoint();
        })
        .assert_user_error("no rewards to claim");
}
