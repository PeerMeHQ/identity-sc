multiversx_sc::imports!();

pub type UserId = usize;

pub const CORE_TOKEN_DECIMALS: u32 = 18;
pub const REWARD_TOKEN_DECIMALS: u32 = 18;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[only_owner]
    #[endpoint(addManager)]
    fn add_manager_endpoint(&self, address: ManagedAddress) {
        self.managers().insert(address);
    }

    #[only_owner]
    #[endpoint(removeManager)]
    fn remove_manager_endpoint(&self, address: ManagedAddress) {
        self.managers().swap_remove(&address);
    }

    #[view(getCoreToken)]
    #[storage_mapper("core_token")]
    fn core_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardToken)]
    #[storage_mapper("reward_token")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAvatarSetCost)]
    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("users")]
    fn users(&self) -> UserMapper;

    #[view(getManagers)]
    #[storage_mapper("managers")]
    fn managers(&self) -> UnorderedSetMapper<ManagedAddress>;
}
