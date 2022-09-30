use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{new_test_ext, Event as TestEvent, KittiesModule, Origin, System, Test};
#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
        let _account_id: u64 = 0;
        System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(0)));
       
		assert_eq!(KittiesModule::next_kitty_id(), 1);
        // let _kitty_id: u32 = NextKittyId::<Test>::get();
        // assert_ne!(Kitties::<Test>::get(_kitty_id),None);
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(0),
            <Test as Config>::KittyPrice::get()
        );
        assert_noop!(KittiesModule::transfer(Origin::signed(2),0,_account_id), Error::<Test>::NotOwner);
	});
}


#[test]
fn create_kitty_fails_for_not_enough_balance() {
	new_test_ext().execute_with(|| {
        let account_id: u64 = 1;
        assert_noop!(
            KittiesModule::create(Origin::signed(account_id)),
            Error::<Test>::NotEnoughBalance
        );
    });
}

#[test]
fn create_kitty_fails_for_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
        let account_id: u64 = 0;
        let max_index = <Test as Config>:: KittyIndex :: max_value();
        NextKittyId::<Test>::set( max_index);
        assert_noop!(
            KittiesModule::create(Origin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    });
}