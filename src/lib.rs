extern crate core;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{current_account_id, prepaid_gas, used_gas};
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, log, near_bindgen, Gas};

const NO_DEPOSIT: u128 = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_fee: u128,
}

impl Default for Contract {
    fn default() -> Self {
        panic!("Gas testing contract")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { contract_fee: 1 }
    }

    fn set_contract_fee(&mut self, fee: U128) {
        self.contract_fee = fee.0;
    }

    /// ccc = cross contract call
    /// template test
    pub fn test1(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
    }

    /// ccc with any gas limit without then
    pub fn test2(&mut self) {
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    /// ccc with_static_gas without then
    pub fn test3(&mut self) {
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 10u64)
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    /// ccc with_unused_gas_weight without then
    pub fn test4(&mut self) {
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(2)
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    /// ccc clear with then
    pub fn test5(&mut self) {
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .plug()
            .then(ext_self::ext(current_account_id())
                .with_attached_deposit(NO_DEPOSIT)
                .plug());
    }

    /// ccc with_static_gas with then
    pub fn test6(&mut self) {
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 10u64)
            .with_attached_deposit(NO_DEPOSIT)
            .plug()
            .then(ext_self::ext(current_account_id())
                .with_static_gas(Gas::ONE_TERA * 10u64)
                .with_attached_deposit(NO_DEPOSIT)
                .plug());
    }

    /// ccc with_unused_gas_weight with then
    pub fn test7(&mut self) {
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(2)
            .with_attached_deposit(NO_DEPOSIT)
            .plug()
            .then(ext_self::ext(current_account_id())
                .with_unused_gas_weight(2)
                .with_attached_deposit(NO_DEPOSIT)
                .plug());
    }

    /// ccc clear nesting 3
    pub fn test8(&mut self) {
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method81();
    }

    /// ccc nesting 3 with_static_gas
    pub fn test9(&mut self) {
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 10u64)
            .with_attached_deposit(NO_DEPOSIT)
            .method91();
    }

    /// ccc nesting 3 with_unused_gas_weight
    pub fn test10(&mut self) {
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(2)
            .with_attached_deposit(NO_DEPOSIT)
            .method101();
    }

    /// ccc with_static_gas can set hard gas
    pub fn test11(&mut self) {
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 25u64)
            .with_attached_deposit(NO_DEPOSIT)
            .plug()
            .then(ext_self::ext(current_account_id())
                .with_static_gas(Gas::ONE_TERA * 75u64)
                .with_attached_deposit(NO_DEPOSIT)
                .plug());
    }
    /// ccc with_static_gas can set hard gas
    pub fn test12(&mut self) {
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(25)
            .with_attached_deposit(NO_DEPOSIT)
            .plug()
            .then(ext_self::ext(current_account_id())
                .with_unused_gas_weight(75)
                .with_attached_deposit(NO_DEPOSIT)
                .plug());
    }

    /// gas burns differently with a large number of cross calls (10)
    pub fn test13(&mut self) {
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method131();
    }

    /// some extra gas after limit (explorer show +5Tgas)
    pub fn test14(&mut self) {
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method141();
    }

    ///
    ///----- Callback methods --------
    ///

    pub fn method81(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method82();
    }

    pub fn method82(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }


    pub fn method91(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 10u64)
            .with_attached_deposit(NO_DEPOSIT)
            .method92();
    }

    pub fn method92(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_static_gas(Gas::ONE_TERA * 10u64)
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    pub fn method101(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(2)
            .with_attached_deposit(NO_DEPOSIT)
            .method82();
    }

    pub fn method102(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_unused_gas_weight(2)
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    pub fn method131(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method132();
    }

    pub fn method132(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method133();
    }

    pub fn method133(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method134();
    }

    pub fn method134(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method135();
    }

    pub fn method135(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method136();
    }

    pub fn method136(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method137();
    }

    pub fn method137(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method138();
    }

    pub fn method138(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .method139();
    }

    pub fn method139(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
        ext_self::ext(current_account_id())
            .with_attached_deposit(NO_DEPOSIT)
            .plug();
    }

    pub fn method141(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        /// ~ 1TGas
        for a in  1..800000 {
            self.set_contract_fee(U128(a));
        }
        log!("{} used gas: {}", line!(), used_gas().0);
    }

    pub fn plug(&mut self) {
        log!("{} prepared gas: {}", line!(), prepaid_gas().0);
        self.set_contract_fee(U128(2));
        log!("{} used gas: {}", line!(), used_gas().0);
    }
}

#[ext_contract(ext_self)]
trait ContractCallbackInterface {
    fn method81(&self);
    fn method82(&self);

    fn method91(&self);
    fn method92(&self);

    fn method101(&self);
    fn method102(&self);

    fn method131(&self);
    fn method132(&self);
    fn method133(&self);
    fn method134(&self);
    fn method135(&self);
    fn method136(&self);
    fn method137(&self);
    fn method138(&self);
    fn method139(&self);

    fn method141(&self);

    fn plug(&self);
}
