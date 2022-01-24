#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct ImageNft<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
}

#[elrond_wasm::contract]
pub trait Identity {
    #[init]
    fn init(&self, cost_token: TokenIdentifier, image_update_cost: BigUint) {
        self.cost_token_id().set_if_empty(&cost_token);
        self.cost_image_set().set_if_empty(&image_update_cost);
    }

    #[payable("*")]
    #[endpoint(setImage)]
    fn set_image(
        &self,
        #[payment_token] cost_token_id: TokenIdentifier,
        #[payment_amount] cost_amount: BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
    ) -> SCResult<()> {
        require!(nft_id.is_valid_esdt_identifier(), "not an nft");
        require!(cost_token_id == self.cost_token_id().get(), "invalid token");
        require!(cost_amount >= self.cost_image_set().get(), "invalid amount");

        self.image_nfts_by_address(&self.blockchain().get_caller()).set(&ImageNft {
            token_id: nft_id,
            nonce: nft_nonce,
        });

        self.burn_cost_tokens(&cost_token_id, &cost_amount);

        Ok(())
    }

    #[only_owner]
    #[endpoint(updateImageSetCost)]
    fn update_image_set_cost_endpoint(&self, cost: BigUint) -> SCResult<()> {
        self.cost_image_set().set(&cost);
        Ok(())
    }

    #[view(getImageNftIdByAddress)]
    fn get_image_nft_id(&self, address: ManagedAddress) -> OptionalResult<MultiResult2<TokenIdentifier, u64>> {
        if (self.image_nfts_by_address(&address).is_empty()) {
            OptionalResult::None
        } else {
            let nft = self.image_nfts_by_address(&address).get();
            OptionalResult::Some((nft.token_id, nft.nonce).into())
        }
    }

    fn burn_cost_tokens(&self, token_id: &TokenIdentifier, amount: &BigUint) {
        self.send().esdt_local_burn(&token_id, 0, &amount);
        self.burned_tokens().update(|current| *current += amount);
    }

    #[storage_mapper("cost_token_id")]
    fn cost_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("cost_image_set")]
    fn cost_image_set(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("images_by_address")]
    fn image_nfts_by_address(&self, address: &ManagedAddress) -> SingleValueMapper<ImageNft<Self::Api>>;

    #[storage_mapper("burned_tokens")]
    fn burned_tokens(&self) -> SingleValueMapper<BigUint>;
}
