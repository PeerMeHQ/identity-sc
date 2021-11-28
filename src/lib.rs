#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct ImageNft<M: ManagedTypeApi> {
    pub token_type: TokenIdentifier<M>,
    pub nonce: u64,
}

#[elrond_wasm::contract]
pub trait Identity {
    #[init]
    fn init(&self, cost_token: TokenIdentifier) {
        self.cost_token_id().set(&cost_token);
    }

    #[payable("*")]
    #[endpoint(setImage)]
    fn set_image(
        &self,
        #[payment_token] cost_token: TokenIdentifier,
        #[payment_amount] cost_amount: BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
    ) -> SCResult<()> {
        require!(nft_id.is_valid_esdt_identifier(), "not an nft");

        let caller = self.blockchain().get_caller();
        let image_nft = ImageNft {
            token_type: nft_id,
            nonce: nft_nonce,
        };

        self.image_nfts_by_address(&caller).set(&image_nft);
        self.send().esdt_local_burn(&cost_token, 0, &cost_amount);

        Ok(())
    }

    #[storage_mapper("costTokenId")]
    fn cost_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("imagesNfts")]
    fn image_nfts_by_address(&self, address: &ManagedAddress) -> SingleValueMapper<ImageNft<Self::Api>>;
}
