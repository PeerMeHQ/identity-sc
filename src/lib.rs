#![no_std]

use config::{CORE_TOKEN_DECIMALS, REWARD_TOKEN_DECIMALS};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod config;
pub mod trust;
pub mod errors;
pub mod earn_proxy;

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Avatar<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
}

#[multiversx_sc::contract]
pub trait Identity: config::ConfigModule + trust::TrustModule {
    #[init]
    fn init(&self, core_token: TokenIdentifier, reward_token: TokenIdentifier, image_update_cost: BigUint) {
        self.core_token().set(&core_token);
        self.reward_token().set(&reward_token);
        self.cost_avatar_set().set(&image_update_cost);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(burnForTrust)]
    fn burn_for_trust_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let core_token = self.core_token().get();
        require!(payment.token_identifier == core_token, "invalid token");

        self.send().esdt_local_burn(&payment.token_identifier, payment.token_nonce, &payment.amount);

        let user = self.get_or_create_trusted_user(&caller);
        let trust = self.calculate_trust_from_tokens(&payment.amount, CORE_TOKEN_DECIMALS);

        let multiplier = if !self.core_token_burn_trust_multiplier().is_empty() {
            self.core_token_burn_trust_multiplier().get() as u64
        } else {
            1u64
        };

        let amplified_trust = trust * multiplier;

        self.increase_trust_score(user, amplified_trust);
    }

    #[payable("*")]
    #[endpoint(migrateToTrust)]
    fn migrate_to_trust_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let reward_token = self.reward_token().get();
        require!(payment.token_identifier == reward_token, "invalid token");

        self.send().esdt_local_burn(&payment.token_identifier, payment.token_nonce, &payment.amount);

        let user = self.get_or_create_trusted_user(&caller);
        let trust = self.calculate_trust_from_tokens(&payment.amount, REWARD_TOKEN_DECIMALS);

        self.increase_trust_score(user, trust);
    }

    #[only_owner]
    #[endpoint(withdrawCostTokens)]
    fn withdraw_cost_tokens_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let core_token_id = self.core_token().get();
        let balance = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(core_token_id.clone()), 0);

        self.send().direct_esdt(&caller, &core_token_id, 0, &balance);
    }

    #[payable("*")]
    #[endpoint(setAvatar)]
    fn set_avatar_endpoint(&self, nft_collection: TokenIdentifier, nft_nonce: u64) {
        let payment = self.call_value().single_esdt();

        require!(nft_collection.is_valid_esdt_identifier(), "not a valid token");
        require!(payment.token_identifier == self.core_token().get(), "invalid token");
        require!(payment.amount >= self.cost_avatar_set().get(), "invalid amount");

        self.avatars(&self.blockchain().get_caller()).set(&Avatar {
            token_id: nft_collection,
            nonce: nft_nonce,
        });

        self.send().esdt_local_burn(&payment.token_identifier, payment.token_nonce, &payment.amount);
    }

    #[only_owner]
    #[endpoint(setAvatarAdmin)]
    fn set_avatar_admin_endpoint(&self, address: ManagedAddress, nft_collection: TokenIdentifier, nft_nonce: u64) {
        require!(nft_collection.is_valid_esdt_identifier(), "not a valid token");

        self.avatars(&address).set(&Avatar {
            token_id: nft_collection,
            nonce: nft_nonce,
        });
    }

    #[endpoint(removeAvatar)]
    fn remove_avatar_endpoint(&self) {
        let caller = self.blockchain().get_caller();

        self.avatars(&caller).clear();
    }

    #[view(getAvatar)]
    fn get_avatar_view(&self, address: ManagedAddress) -> OptionalValue<MultiValue2<TokenIdentifier, u64>> {
        if self.avatars(&address).is_empty() {
            OptionalValue::None
        } else {
            let nft = self.avatars(&address).get();
            OptionalValue::Some((nft.token_id, nft.nonce).into())
        }
    }

    #[storage_mapper("avatars")]
    fn avatars(&self, address: &ManagedAddress) -> SingleValueMapper<Avatar<Self::Api>>;
}
