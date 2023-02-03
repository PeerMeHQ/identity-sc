multiversx_sc::imports!();

use multiversx_sc_scenario::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_distributes_to_core_stakers() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();
    let stake_total = 20u32;

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(stake_total), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            let expected_rtp = managed_biguint!(500) * managed_biguint!(10).pow(18) / stake_total;

            sc.distribute_to_earn_core_endpoint();

            assert_eq!(sc.core_reward_per_token().get(), expected_rtp);
        })
        .assert_ok();

    // test it adds up
    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            let expected_rtp = sc.core_reward_per_token().get() + managed_biguint!(500) * managed_biguint!(10).pow(18) / stake_total;

            sc.distribute_to_earn_core_endpoint();

            assert_eq!(sc.core_reward_per_token().get(), expected_rtp);
        })
        .assert_ok();
}

#[test]
fn it_fails_if_paid_with_wrong_token() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, b"TOKEN-123456", &rust_biguint!(500));

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, b"TOKEN-123456", 0, &rust_biguint!(500), |sc| {
            sc.distribute_to_earn_core_endpoint();
        })
        .assert_user_error("invalid token");
}
