/////////////////////////////////////////////////////////////////////////
// 0L Module
// CASES of validation and mining
/////////////////////////////////////////////////////////////////////////

/// # Summary
/// This module can be used by root to determine whether a validator is compliant
/// Validators who are no longer compliant may be kicked out of the validator
/// set and/or jailed. To be compliant, validators must be BOTH validating and mining.
module ol_framework::grade {
    use diem_framework::stake;
    use std::fixed_point32::{Self, FixedPoint32};


    /// minimum number of accepted proposals vs. rounds to qualify as compliant
    const THRESHOLD_OF_PROPOSALS_IN_ROUND: u64 = 5;

    #[view]
    /// returns if the validator passed or failed, and the number of proposals
    /// and failures, and the ratio.
    /// returns: is the validator compliant, proposed blocks, failed blocks, and the ratio of proposed to failed.

    public fun get_validator_grade(node_addr: address, epoch_rounds: u64): (bool, u64, u64, FixedPoint32) {
      let idx = stake::get_validator_index(node_addr);
      let (proposed, failed) = stake::get_current_epoch_proposal_counts(idx);

      let more_proposed_than_failed = proposed > failed;

      let ratio_to_rounds = fixed_point32::create_from_rational(proposed, epoch_rounds);
      let percent_of_rounds = fixed_point32::multiply_u64(100, ratio_to_rounds);

      let above_thresh = percent_of_rounds > THRESHOLD_OF_PROPOSALS_IN_ROUND;


      let compliant = more_proposed_than_failed && above_thresh;
      // make failed at leat 1 to avoid division by zero
      (compliant, proposed, failed, fixed_point32::create_from_rational(proposed, (failed + 1)))
    }

}
