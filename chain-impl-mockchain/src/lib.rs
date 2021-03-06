#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate chain_addr;

pub mod block;
#[cfg(test)]
pub mod environment;
pub mod error;
pub mod key;
pub mod leadership;
pub mod ledger;
pub mod setting;
pub mod transaction;
pub mod update;

#[cfg(test)]
mod tests {
    use crate::ledger::Ledger;
    use crate::transaction::SignedTransaction;
    use chain_core::property::testing;
    use quickcheck::TestResult;

    quickcheck! {
        /// Randomly generated transaction should fail.
        fn prop_bad_tx_fails(l: Ledger, tx: SignedTransaction) -> TestResult {
            testing::prop_bad_transaction_fails(l, tx)
        }
    }

}
