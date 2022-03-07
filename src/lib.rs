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
    fn set_avatar_endpoint(
        &self,
        #[payment_token] cost_token_id: TokenIdentifier,
        #[payment_amount] cost_amount: BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
    ) {
        require!(nft_id.is_valid_esdt_identifier(), "not an nft");
        require!(cost_token_id == self.cost_token_id().get(), "invalid token");
        require!(cost_amount >= self.cost_avatar_set().get(), "invalid amount");

        self.avatars(&self.blockchain().get_caller()).set(&Avatar {
            token_id: nft_id,
            nonce: nft_nonce,
        });

        // self.burn_cost_tokens(&cost_token_id, &cost_amount);
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

    // fn burn_cost_tokens(&self, token_id: &TokenIdentifier, amount: &BigUint) {
    //     self.send().esdt_local_burn(&token_id, 0, &amount);
    //     self.burned_tokens().update(|current| *current += amount);
    // }

    #[storage_mapper("cost_token_id")]
    fn cost_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("cost_avatar_set")]
    fn cost_avatar_set(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("avatars")]
    fn avatars(&self, address: &ManagedAddress) -> SingleValueMapper<Avatar<Self::Api>>;

    #[storage_mapper("burned_tokens")]
    fn burned_tokens(&self) -> SingleValueMapper<BigUint>;
}
