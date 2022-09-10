#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Avatar<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
}

#[elrond_wasm::contract]
pub trait Identity {
    #[init]
    fn init(&self, cost_token: TokenIdentifier, image_update_cost: BigUint) {
        self.cost_token_id().set_if_empty(&cost_token);
        self.cost_avatar_set().set_if_empty(&image_update_cost);
    }

    #[payable("*")]
    #[endpoint(setAvatar)]
    fn set_avatar_endpoint(&self, nft_collection: TokenIdentifier, nft_nonce: u64) {
        let payment = self.call_value().single_esdt();

        require!(nft_collection.is_valid_esdt_identifier(), "not a valid token");
        require!(payment.token_identifier == self.cost_token_id().get(), "invalid token");
        require!(payment.amount >= self.cost_avatar_set().get(), "invalid amount");

        self.avatars(&self.blockchain().get_caller()).set(&Avatar {
            token_id: nft_collection,
            nonce: nft_nonce,
        });
    }

    #[only_owner]
    #[endpoint(updateAvatarSetCost)]
    fn update_image_set_cost_endpoint(&self, cost: BigUint) {
        self.cost_avatar_set().set(&cost);
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

    #[storage_mapper("cost_token_id")]
    fn cost_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAvatarSetCost)]
    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("avatars")]
    fn avatars(&self, address: &ManagedAddress) -> SingleValueMapper<Avatar<Self::Api>>;
}
