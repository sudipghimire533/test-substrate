use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn sample_transfer() {
    new_test_ext().execute_with(|| {
        let sender = 100;
        let receiver = 101;

        // Let's put some balance in sender account first
        assert_ok!(<Test as crate::Config>::Money::set_balance(Origin::root(), sender, 10000, 0));

        // Record balance of sender before doing our transfer
        let sender_balance_before_transfer = <Test as crate::Config>::Money::free_balance(&sender);
        // Record balance of receiver before transfer
        let receiver_balance_before_transfer = <Test as crate::Config>::Money::free_balance(&receiver);

        // Call out escrow transfer function
        assert_ok!(TemplateModule::escrow_transfer(Origin::signed(sender), receiver, 500));

        // Sender balance should decrease by 500
        let expected_sender_balance = sender_balance_before_transfer - 500;
        // receover balance should increase by 500
        let expected_receiver_balance = receiver_balance_before_transfer + 500;

        // check expectiations
        assert_eq!(expected_sender_balance, <Test as crate::Config>::Money::free_balance(&sender));
        assert_eq!(expected_receiver_balance, <Test as crate::Config>::Money::free_balance(&receiver));
    });
}
