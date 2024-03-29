multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[storage_mapper("core_token")]
    fn core_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAvatarSetCost)]
    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;
}
