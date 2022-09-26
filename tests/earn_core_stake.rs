elrond_wasm::imports!();

use elrond_wasm_debug::*;
use identity::earn::EarnModule;
use setup::*;

mod setup;

#[test]
fn it_stakes_core_stake_tokens() {
    let mut setup = setup::setup_contract(identity::contract_obj);
    let user_address = setup.user_address.clone();

    setup.blockchain.set_esdt_balance(&user_address, EARN_STAKE_CORE_TOKEN_ID, &rust_biguint!(1_000));

    setup
        .blockchain
        .execute_esdt_transfer(&user_address, &setup.contract, EARN_STAKE_CORE_TOKEN_ID, 0, &rust_biguint!(500), |sc| {
            sc.stake_for_earn_endpoint();

            assert_eq!(sc.core_stake(&managed_address!(&user_address)).get(), managed_biguint!(500));
            assert_eq!(sc.core_stake_total().get(), managed_biguint!(500));
        })
        .assert_ok();
}
