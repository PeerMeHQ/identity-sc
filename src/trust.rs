multiversx_sc::imports!();

use crate::{config::{self, UserId}, errors::{ERR_TRUST_BANNED, ERR_TRUST_CALLER_NOT_MANAGER, ERR_USER_NOT_FOUND}};

const START_AMOUNT: u64 = 1;
const BAN_THRESHOLD: u64 = 0;

pub const CORE_TOKEN_BURN_TRUST_MULTIPLIER: u64 = 2;

#[multiversx_sc::module]
pub trait TrustModule: config::ConfigModule {
    #[endpoint(addTrust)]
    fn add_trust_endpoint(&self, address: ManagedAddress, amount: u64) {
        self.require_caller_trust_manager();

        let user_id = self.users().get_user_id(&address);

        self.increase_trust_score(user_id, amount);
    }

    #[endpoint(banUser)]
    fn ban_user_endpoint(&self, address: ManagedAddress) {
        self.require_caller_trust_manager();

        let user_id = self.get_or_create_trusted_user(&address);

        self.trust_score(user_id).set(BAN_THRESHOLD);
    }

    fn require_not_banned(&self, user: UserId) {
        let trust_score = self.trust_score(user).get();

        require!(trust_score > BAN_THRESHOLD, ERR_TRUST_BANNED);
    }

    fn increase_trust_score(&self, user: UserId, amount: u64) {
        require!(user != 0, ERR_USER_NOT_FOUND);

        self.trust_score(user).update(|trust_score| *trust_score += amount);
    }

    fn get_trusted_user_or_fail(&self, address: &ManagedAddress) -> usize {
        let user = self.get_or_create_trusted_user(address);

        require!(user != 0, ERR_USER_NOT_FOUND);
        self.require_not_banned(user);

        user
    }

    fn get_or_create_trusted_user(&self, address: &ManagedAddress) -> usize {
        let user = self.users().get_user_id(address);

        if user != 0 {
            return user;
        }

        let new_user = self.users().get_or_create_user(address);

        self.trust_score(new_user).set(START_AMOUNT);

        new_user
    }

    fn require_caller_trust_manager(&self) {
        let caller = self.blockchain().get_caller();
        let is_owner = self.blockchain().get_owner_address() == caller;
        let is_manager = self.managers().contains(&caller);

        require!(is_owner || is_manager, ERR_TRUST_CALLER_NOT_MANAGER);
    }

    fn calculate_trust_from_tokens(&self, amount: &BigUint, decimals: u32) -> u64 {
        let amount = amount / &BigUint::from(10u64).pow(decimals);

        amount.to_u64().unwrap()
    }

    #[storage_mapper("trust:score")]
    fn trust_score(&self, user: UserId) -> SingleValueMapper<u64>;
}
