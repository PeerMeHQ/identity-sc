multiversx_sc::imports!();

use multiversx_sc_scenario::*;
use identity::config::*;
use identity::*;
use setup::*;

mod setup;

#[test]
fn it_removes_an_avatar() {
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
        .execute_tx(&setup.user_address, &setup.contract, &rust_biguint!(0), |sc| {
            sc.remove_avatar_endpoint();
        })
        .assert_ok();

    setup
        .blockchain
        .execute_query(&setup.contract, |sc| {
            assert!(sc.avatars(&managed_address!(&user_address)).is_empty());
        })
        .assert_ok();
}
