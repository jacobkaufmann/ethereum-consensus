// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::Eth1DataResetTestCase;
use crate::test_utils::TestCase;
use ethereum_consensus::phase0::minimal as spec;

#[test]
fn test_eth_1_vote_no_reset() {
    let mut test_case = Eth1DataResetTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/eth1_data_reset/pyspec_tests/eth1_vote_no_reset");

    test_case.execute(|state, context| {
        spec::process_eth1_data_reset(state, context);
        Ok(())
    });
}

#[test]
fn test_eth_1_vote_reset() {
    let mut test_case = Eth1DataResetTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/eth1_data_reset/pyspec_tests/eth1_vote_reset");

    test_case.execute(|state, context| {
        spec::process_eth1_data_reset(state, context);
        Ok(())
    });
}