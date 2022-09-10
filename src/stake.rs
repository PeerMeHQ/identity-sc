elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait StakeModule {
    #[endpoint(configureStaking)]
    fn configure_staking_endpoint(&self) {}

    #[endpoint(stake)]
    fn stake_endpoint(&self) {}

    #[endpoint(unstake)]
    fn unstake_endpoint(&self) {}

    #[storage_mapper("stake_token")]
    fn stake_token() -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("stake_min_amount")]
    fn stake_min_amount() -> SingleValueMapper<BigUint>;

    #[storage_mapper("stake_lock_time")]
    fn stake_lock_time() -> SingleValueMapper<u64>;

    #[storage_mapper("staked_total_amount")]
    fn staked_total_amount() -> SingleValueMapper<BigUint>;

    #[storage_mapper("staked_amount")]
    fn staked_amount(address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
