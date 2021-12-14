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
        self.cost_token_id().set(&cost_token);
        self.image_update_cost().set(&image_update_cost);
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
        require!(cost_amount >= self.image_update_cost().get(), "invalid amount");

        let caller = self.blockchain().get_caller();
        let image_nft = ImageNft {
            token_id: nft_id,
            nonce: nft_nonce,
        };

        self.image_nfts_by_address(&caller).set(&image_nft);

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

    #[storage_mapper("cost_token_id")]
    fn cost_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("cost_image_update")]
    fn image_update_cost(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("images_by_address")]
    fn image_nfts_by_address(&self, address: &ManagedAddress) -> SingleValueMapper<ImageNft<Self::Api>>;
}
