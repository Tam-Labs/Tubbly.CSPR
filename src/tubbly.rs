//!
//! Smart Contract for Requesting Balance Change and Confirmation
//!
//! This smart contract allows users to request changes to their balance
//! and requires confirmation by the contract owner before the changes can be executed.
//! It provides a secure and transparent way to manage balance adjustments within the contract.
//!
//! The contract owner has the authority to approve or reject balance change requests made by users.
//! Only after confirmation by the contract owner will the requested changes be applied to the user's balance.
//!
//! Features:
//! - Users can submit requests to increase or decrease their balance.
//! - Requests are pending until confirmed by the contract owner.
//! - The contract owner has the sole authority to confirm or reject balance change requests.
//! - Once confirmed, the requested balance changes are applied to the user's balance.
//! - Provides transparency and accountability in managing balance adjustments.
//!
//! Usage:
//! 1. Users initiate a balance change request by calling the appropriate function with the desired amount.
//! 2. The request is added to the pending list until confirmed by the contract owner.
//! 3. The contract owner reviews pending requests and can confirm them.
//! 4. Confirmed requests are executed, and the user's balance is updated accordingly.
//!
//! Note: This contract assumes that the contract owner is a trusted entity with the authority to manage balance changes within the contract.
use odra::prelude::*;
use odra::Mapping;
use odra::Var;
use odra::{
    casper_types::{U128, U256},
    Address, UnwrapOrRevert,
};

type RequestId = U128;
type BalanceType = U256;

#[odra::module(events = [OwnershipChanged, Submission, Confirmation])]
pub struct Tubbly {
    owner: Var<Option<Address>>,
    balances: Mapping<Address, BalanceType>,
    requests: Mapping<RequestId, Option<Request>>,
    next_id: Var<RequestId>,
}

#[odra::odra_type]
pub struct Request {
    caller: Address,
    balance: BalanceType,
}

#[odra::odra_error]
pub enum Error {
    NotOwner = 1,
    OwnerIsAlreadyInitialized = 2,
    OwnerIsNotInitialized = 3,
    IncorrectRequestId = 4,
    RequestIdExhausted = 5,
}

#[odra::event]
pub struct OwnershipChanged {
    pub prev_owner: Option<Address>,
    pub new_owner: Address,
}

#[odra::event]
pub struct Submission {
    pub req_id: RequestId,
}

#[odra::event]
pub struct Confirmation {
    pub req_id: RequestId,
}

#[odra::module]
impl Tubbly {
    pub fn init(&mut self, owner: Address) {
        if self.owner.get_or_default().is_some() {
            self.env().revert(Error::OwnerIsAlreadyInitialized)
        }

        self.owner.set(Some(owner));
        self.next_id.set(RequestId::from(0));

        self.env().emit_event(OwnershipChanged {
            prev_owner: None,
            new_owner: owner,
        });
    }

    pub fn submit(&mut self, balance: BalanceType) -> RequestId {
        let env = self.env();
        let req_id = self.next_id.get_or_revert_with(Error::IncorrectRequestId);

        self.next_id.set(
            req_id
                .checked_add(RequestId::from(1))
                .unwrap_or_revert_with(&env, Error::RequestIdExhausted),
        );

        let caller = env.caller();
        let request = Request { caller, balance };

        self.requests.set(&req_id, Some(request));

        env.emit_event(Submission { req_id });

        req_id
    }

    pub fn confirm(&mut self, req_id: RequestId) -> BalanceType {
        let env = self.env();
        let caller = env.caller();

        self.ensure_ownership(&caller);

        let request = self
            .take_request(req_id)
            .unwrap_or_revert_with(&env, Error::IncorrectRequestId);

        self.balances.set(&request.caller, request.balance);

        env.emit_event(Confirmation { req_id });

        request.balance
    }

    pub fn balance_of(&mut self, address: Address) -> BalanceType {
        self.balances.get_or_default(&address)
    }

    pub fn get_request(&self, req_id: RequestId) -> Option<Request> {
        let env = self.env();
        let caller = env.caller();

        self.ensure_ownership(&caller);

        self.requests.get_or_default(&req_id)
    }

    pub fn ensure_ownership(&self, address: &Address) {
        if Some(address) != self.owner.get_or_default().as_ref() {
            self.env().revert(Error::NotOwner)
        }
    }

    pub fn change_ownership(&mut self, new_owner: &Address) {
        let env = self.env();

        self.ensure_ownership(&env.caller());

        let current_owner = self.get_owner();
        self.owner.set(Some(*new_owner));

        env.emit_event(OwnershipChanged {
            prev_owner: Some(current_owner),
            new_owner: *new_owner,
        });
    }

    pub fn get_owner(&self) -> Address {
        match self.owner.get_or_default() {
            Some(owner) => owner,
            None => self.env().revert(Error::OwnerIsNotInitialized),
        }
    }

    fn take_request(&mut self, req_id: RequestId) -> Option<Request> {
        let request = self.requests.get_or_default(&req_id);
        if request.is_some() {
            self.requests.set(&req_id, None);
        }
        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostEnv, HostRef};

    fn setup() -> (TubblyHostRef, HostEnv, Address) {
        let env: HostEnv = odra_test::env();
        let init_args = TubblyInitArgs {
            owner: env.get_account(0),
        };
        (
            TubblyHostRef::deploy(&env, init_args),
            env.clone(),
            env.get_account(0),
        )
    }

    #[test]
    fn initialization_works() {
        let (ownable, env, owner) = setup();
        assert_eq!(ownable.get_owner(), owner);

        assert!(env.emitted_event(
            ownable.address(),
            &OwnershipChanged {
                prev_owner: None,
                new_owner: owner,
            },
        ));
    }

    #[test]
    fn owner_can_change_ownership() {
        let (mut ownable, env, owner) = setup();
        let new_owner = env.get_account(1);

        env.set_caller(owner);
        ownable.change_ownership(&new_owner);
        assert_eq!(ownable.get_owner(), new_owner);

        assert!(env.emitted_event(
            ownable.address(),
            &OwnershipChanged {
                prev_owner: Some(owner),
                new_owner,
            },
        ));
    }

    #[test]
    fn non_owner_cannot_change_ownership() {
        let (mut ownable, env, _) = setup();
        let new_owner = env.get_account(1);
        ownable.change_ownership(&new_owner);

        assert_eq!(
            ownable.try_change_ownership(&new_owner),
            Err(Error::NotOwner.into())
        );
    }

    #[test]
    fn submit_works() {
        let balance = BalanceType::from(100);

        let (mut ownable, env, owner) = setup();

        let req_id = ownable.submit(balance);

        let request = ownable.get_request(req_id);
        assert!(request.is_some());

        let req = request.unwrap();
        assert_eq!(req.caller, owner);
        assert_eq!(req.balance, balance);

        assert!(env.emitted_event(&ownable.address(), &Submission { req_id }));
    }

    #[test]
    fn confirm_works() {
        let balance = BalanceType::from(100);

        let (mut ownable, env, owner) = setup();

        let new_owner = env.get_account(1);
        env.set_caller(new_owner);
        let req_id = ownable.submit(balance);

        env.set_caller(owner);
        let request = ownable.get_request(req_id);
        assert!(request.is_some());

        let confirmed_balance = ownable.confirm(req_id);

        let req = request.unwrap();
        assert_eq!(req.caller, new_owner);
        assert_eq!(req.balance, balance);
        assert_eq!(req.balance, confirmed_balance);

        assert!(env.emitted_event(&ownable.address(), &Confirmation { req_id }));

        let new_owner_balance = ownable.balance_of(new_owner);
        assert_eq!(balance, new_owner_balance);
    }
}
