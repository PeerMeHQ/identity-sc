#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod config;

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Avatar<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
}

#[multiversx_sc::contract]
pub trait Identity: config::ConfigModule {
    #[init]
    fn init(&self, core_token: TokenIdentifier, image_update_cost: BigUint) {
        self.core_token().set_if_empty(&core_token);
        self.cost_avatar_set().set(&image_update_cost);
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
