use crate::errors::ERR_CALLER_NOT_MANAGER;

multiversx_sc::imports!();

pub type UserId = usize;
pub type TrustPoints = u64;

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

    #[only_owner]
    #[endpoint(setCoreTokenBurnTrustMultiplier)]
    fn set_core_token_burn_trust_multiplier_endpoint(&self, multiplier: u8) {
        self.core_token_burn_trust_multiplier().set(multiplier);
    }

    fn require_caller_manager(&self) {
        let caller = self.blockchain().get_caller();
        let is_owner = self.blockchain().get_owner_address() == caller;
        let is_manager = self.managers().contains(&caller);

        require!(is_owner || is_manager, ERR_CALLER_NOT_MANAGER);
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

    #[view(getCoreTokenBurnTrustMultiplier)]
    #[storage_mapper("core_token_burn_trust_multiplier")]
    fn core_token_burn_trust_multiplier(&self) -> SingleValueMapper<u8>;
}
