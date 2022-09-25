elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ConfigModule {
    #[storage_mapper("core_token")]
    fn core_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAvatarSetCost)]
    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;

    // #[storage_mapper("earn:stake_min_amount")]
    // fn stake_min_amount(&self) -> SingleValueMapper<BigUint>;

    // #[storage_mapper("earn:stake_lock_time")]
    // fn stake_lock_time(&self) -> SingleValueMapper<u64>;
}
