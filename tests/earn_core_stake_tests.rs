multiversx_sc::imports!();

use multiversx_sc_scenario::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_stakes_core_stake_tokens() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            sc.stake_for_earn_endpoint();

            assert_eq!(sc.core_stake(&managed_address!(&user_address)).get(), managed_biguint!(500));
            assert_eq!(sc.core_stake_total().get(), managed_biguint!(500));
        })
        .assert_ok();
}
