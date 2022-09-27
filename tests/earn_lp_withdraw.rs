elrond_wasm::imports!();

use elrond_wasm_debug::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_unstakes_lp_stake_tokens() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, EARN_STAKE_LP_TOKEN_ID, &rust_biguint!(1_000));

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_LP_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            sc.stake_for_earn_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_tx(&user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.withdraw_from_earn_endpoint(managed_token_id!(EARN_STAKE_LP_TOKEN_ID), managed_biguint!(500));

            assert_eq!(sc.lp_stake(&managed_address!(&user_address)).get(), managed_biguint!(0));
            assert_eq!(sc.lp_stake_total().get(), managed_biguint!(0));
        })
        .assert_ok();
}
