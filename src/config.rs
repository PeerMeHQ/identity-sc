multiversx_sc::imports!();

pub type UserId = usize;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[storage_mapper("core_token")]
    fn core_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("reward_token")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAvatarSetCost)]
    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("users")]
    fn users(&self) -> UserMapper;
}
