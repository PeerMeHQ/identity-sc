multiversx_sc::imports!();

use crate::{config::{self, TrustPoints, UserId}, earn_proxy, errors::{ERR_TRUST_BANNED, ERR_USER_NOT_FOUND}};

const START_AMOUNT: u64 = 1;
const BAN_THRESHOLD: u64 = 0;
const STAKE_THRESHOLD: u64 = 500;

#[multiversx_sc::module]
pub trait TrustModule: config::ConfigModule {
    #[only_owner]
    #[endpoint(setTrustStakeContract)]
    fn set_trust_stake_contract_endpoint(&self, contract: ManagedAddress, entity: ManagedAddress) {
        require!(self.blockchain().is_smart_contract(&contract), "invalid contract");

        self.trust_stake_contract().set(contract);
        self.trust_stake_contract_entity().set(entity);
    }

    #[endpoint(addTrust)]
    fn add_trust_endpoint(&self, user: ManagedAddress, amount: TrustPoints) {
        self.require_caller_manager();

        let user_id = self.get_or_create_trusted_user(&user);

        self.increase_trust_score(user_id, amount);
    }

    #[endpoint(addTrustBatch)]
    fn add_trust_batch_endpoint(&self, entries: MultiValueEncoded<MultiValue2<ManagedAddress, TrustPoints>>) {
        self.require_caller_manager();

        for entry in entries.into_iter() {
            let (address, trust) = entry.into_tuple();

            let user_id = self.get_or_create_trusted_user(&address);

            self.increase_trust_score(user_id, trust);
        }
    }

    #[endpoint(banUser)]
    fn ban_user_endpoint(&self, address: ManagedAddress) {
        self.require_caller_manager();

        let user = self.get_or_create_trusted_user(&address);

        self.trust_score(user).set(BAN_THRESHOLD);

        self.renew_trust_stake(user, 0);
    }

    #[endpoint(restakeTrust)]
    fn restake_trust_endpoint(&self, user: ManagedAddress) {
        let user = self.users().get_user_id(&user);
        require!(user != 0, ERR_USER_NOT_FOUND);

        let trust = self.trust_score(user).get();

        self.renew_trust_stake(user, trust);
    }

    #[view(getTrustScore)]
    fn get_trust_score(&self, address: ManagedAddress) -> u64 {
        let user = self.users().get_user_id(&address);

        if user == 0 {
            return 0;
        }

        self.trust_score(user).get()
    }

    fn require_not_banned(&self, user: UserId) {
        let trust_score = self.trust_score(user).get();

        require!(trust_score > BAN_THRESHOLD, ERR_TRUST_BANNED);
    }

    fn increase_trust_score(&self, user: UserId, trust: TrustPoints) {
        require!(user != 0, ERR_USER_NOT_FOUND);

        let trust_score = self.trust_score(user).get();
        let new_trust_score = trust_score + trust;
        self.trust_score(user).set(new_trust_score);

        self.renew_trust_stake(user, new_trust_score);
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

    fn calculate_trust_from_tokens(&self, amount: &BigUint, decimals: u32) -> u64 {
        let amount = amount / &BigUint::from(10u64).pow(decimals);

        amount.to_u64().unwrap()
    }

    fn renew_trust_stake(&self, user: UserId, trust: TrustPoints) {
        if trust != BAN_THRESHOLD && trust < STAKE_THRESHOLD {
            return;
        }

        let contract = self.trust_stake_contract().get();
        let entity = self.trust_stake_contract_entity().get();
        let user = self.users().get_user_address_unchecked(user);

        self.tx()
            .to(&contract)
            .typed(earn_proxy::EarnProxy)
            .renew_virtual_stake_endpoint(entity, user, trust)
            .sync_call();
    }

    #[storage_mapper("trust:score")]
    fn trust_score(&self, user: UserId) -> SingleValueMapper<u64>;

    #[storage_mapper("trust:stake_contract")]
    fn trust_stake_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("trust:stake_contract_entity")]
    fn trust_stake_contract_entity(&self) -> SingleValueMapper<ManagedAddress>;
}
