elrond_wasm::imports!();

use crate::stake;

#[elrond_wasm::module]
pub trait EarnModule: stake::StakeModule {
    #[endpoint(supplyLp)]
    fn supply_lp_earnings(&self) {}

    #[endpoint(fuelLpEarnings)]
    fn fuel_lp_earnings_endpoint(&self) {}

    #[endpoint(fuelBoostEarnings)]
    fn fuel_boost_earnings_endpoint(&self) {}

    #[endpoint(claim)]
    fn claim_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let earnings_amount = self.calculate_earnings();

        // self.send().direct_esdt(&caller, token, 0, earnings_amount, data);
    }

    fn calculate_earnings(&self) -> BigUint {
        let caller = self.blockchain().get_caller();
        let staked_total = self.staked_total_amount().get();
        let staked_caller = self.staked_amount(&caller).get();
        let pool_share_percent = (staked_caller * 100) / staked_total;
    }
}
