elrond_wasm::imports!();

use crate::config;

const PRECISION: u32 = 18;

#[elrond_wasm::module]
pub trait EarnModule: config::ConfigModule {
    #[only_owner]
    #[endpoint(initEarnModule)]
    fn init_earn_module_endpoint(&self, core_stake_token: TokenIdentifier, lp_stake_token: TokenIdentifier, lock_time_seconds: u64) {
        self.core_stake_token().set(core_stake_token);
        self.lp_stake_token().set(lp_stake_token);
        self.lock_time_seconds().set(lock_time_seconds);
    }

    #[payable("*")]
    #[endpoint(distributeToCore)]
    fn distribute_to_earn_core_endpoint(&self) {
        let payment = self.call_value().single_esdt();
        let core_token = self.core_token().get();
        let core_stake_total = self.core_stake_total().get();

        require!(payment.amount > 0, "invalid amount");
        require!(payment.token_identifier == core_token, "invalid token");
        require!(core_stake_total >= 0, "total stake must be more than 0");

        self.core_reward_per_token()
            .update(|current| *current += payment.amount.clone() * BigUint::from(10u64).pow(PRECISION) / core_stake_total);
    }

    #[payable("*")]
    #[endpoint(distributeToLps)]
    fn distribute_to_earn_lp_endpoint(&self) {
        let payment = self.call_value().single_esdt();
        let core_token = self.core_token().get();
        let lp_stake_total = self.lp_stake_total().get();

        require!(payment.amount > 0, "invalid amount");
        require!(payment.token_identifier == core_token, "invalid token");
        require!(lp_stake_total >= 0, "total stake must be more than 0");

        self.lp_reward_per_token()
            .update(|current| *current += payment.amount.clone() * BigUint::from(10u64).pow(PRECISION) / lp_stake_total);
    }

    #[payable("*")]
    #[endpoint(stakeForEarn)]
    fn stake_for_earn_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        self.lock_stake_for(&caller);

        if payment.token_identifier == self.core_stake_token().get() {
            let rpt = self.core_reward_per_token().get();
            self.core_stake(&caller).update(|current| *current += payment.amount.clone());
            self.core_reward_tally(&caller).update(|curr| *curr += BigInt::from(rpt * payment.amount.clone()));
            self.core_stake_total().update(|current| *current += payment.amount);
        } else if payment.token_identifier == self.lp_stake_token().get() {
            let rpt = self.lp_reward_per_token().get();
            self.lp_stake(&caller).update(|current| *current += payment.amount.clone());
            self.lp_reward_tally(&caller).update(|current| *current += BigInt::from(rpt * payment.amount.clone()));
            self.lp_stake_total().update(|current| *current += payment.amount);
        } else {
            sc_panic!("invalid stake token");
        }
    }

    #[endpoint(withdrawFromEarn)]
    fn withdraw_from_earn_endpoint(&self, token: TokenIdentifier, amount: BigUint) {
        let caller = self.blockchain().get_caller();
        self.require_stake_unlocked_for(&caller);

        if token == self.core_stake_token().get() {
            let stake = self.core_stake(&caller).get();
            let rpt = self.core_reward_per_token().get();
            require!(stake > 0, "no stake by user");
            require!(amount <= stake, "invalid amount");
            self.core_stake(&caller).update(|current| *current -= amount.clone());
            self.core_stake_total().update(|current| *current -= amount.clone());
            self.core_reward_tally(&caller).update(|current| *current -= BigInt::from(rpt * amount.clone()));
            self.send().direct_esdt(&caller, &token, 0, &amount);
        } else if token == self.lp_stake_token().get() {
            let stake = self.lp_stake(&caller).get();
            let rpt = self.lp_reward_per_token().get();
            require!(stake > 0, "no stake by user");
            require!(amount <= stake, "invalid amount");
            self.lp_stake(&caller).update(|current| *current -= amount.clone());
            self.lp_stake_total().update(|current| *current -= amount.clone());
            self.lp_reward_tally(&caller).update(|current| *current -= BigInt::from(rpt * amount.clone()));
            self.send().direct_esdt(&caller, &token, 0, &amount);
        } else {
            sc_panic!("invalid token");
        }
    }

    #[endpoint(claimEarnRewards)]
    fn claim_earn_reward_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let reward_total = self.compute_reward(&caller);

        require!(reward_total > 0, "no rewards to claim");

        let core_stake = self.core_stake(&caller).get();
        let core_rpt = self.core_reward_per_token().get();
        self.core_reward_tally(&caller).set(BigInt::from(core_stake * core_rpt));

        let lp_stake = self.lp_stake(&caller).get();
        let lp_rpt = self.lp_reward_per_token().get();
        self.lp_reward_tally(&caller).set(BigInt::from(lp_stake * lp_rpt));

        let core_token = self.core_token().get();
        self.send().direct_esdt(&caller, &core_token, 0, &reward_total);
    }

    #[view(getEarnerInfo)]
    fn get_earner_info_view(&self, address: ManagedAddress) -> MultiValue4<BigUint, BigUint, BigUint, u64> {
        let core_stake = self.core_stake(&address).get();
        let lp_stake = self.lp_stake(&address).get();
        let reward_total = self.compute_reward(&address);
        let unlock_time = self.unlock_time(&address).get();

        (core_stake, lp_stake, reward_total, unlock_time).into()
    }

    fn compute_reward(&self, address: &ManagedAddress) -> BigUint {
        let core_staked = self.core_stake(&address).get();
        let core_reward_per_token = self.core_reward_per_token().get();
        let core_reward_tally = self.core_reward_tally(&address).get();
        let core_reward = BigInt::from(core_staked * core_reward_per_token) - core_reward_tally;
        let core_reward = core_reward.into_big_uint().unwrap_or_else(|| BigUint::zero());

        let lp_staked = self.lp_stake(&address).get();
        let lp_reward_per_token = self.lp_reward_per_token().get();
        let lp_reward_tally = self.lp_reward_tally(&address).get();
        let lp_reward = BigInt::from(lp_staked * lp_reward_per_token) - lp_reward_tally;
        let lp_reward = lp_reward.into_big_uint().unwrap_or_else(|| BigUint::zero());

        (core_reward + lp_reward) / BigUint::from(10u64).pow(PRECISION)
    }

    fn lock_stake_for(&self, address: &ManagedAddress) {
        let lock_until = self.blockchain().get_block_timestamp() + self.lock_time_seconds().get();
        self.unlock_time(&address).set(lock_until);
    }

    fn require_stake_unlocked_for(&self, address: &ManagedAddress) {
        let current_time = self.blockchain().get_block_timestamp();
        let unlock_time = self.unlock_time(&address).get();
        require!(unlock_time > 0, "nothing to unlock");
        require!(current_time > unlock_time, "stake is locked");
    }

    #[storage_mapper("earn:lock_time_seconds")]
    fn lock_time_seconds(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("earn:unlock_time")]
    fn unlock_time(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // --

    #[storage_mapper("earn:core_stake_token")]
    fn core_stake_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("earn:core_stake_total")]
    fn core_stake_total(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_stake")]
    fn core_stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_per_token")]
    fn core_reward_per_token(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_tally")]
    fn core_reward_tally(&self, address: &ManagedAddress) -> SingleValueMapper<BigInt>;

    // --

    #[storage_mapper("earn:lp_stake_token")]
    fn lp_stake_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("earn:lp_stake_total")]
    fn lp_stake_total(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_stake")]
    fn lp_stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_reward_per_token")]
    fn lp_reward_per_token(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_reward_tally")]
    fn lp_reward_tally(&self, address: &ManagedAddress) -> SingleValueMapper<BigInt>;
}
