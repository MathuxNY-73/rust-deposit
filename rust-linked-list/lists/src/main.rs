use lists::borrow_checker;
use lists::sixth::assert_properties;

fn main() {
    // borrow_checker::basic_borrows();
    // borrow_checker::basic_unsafe_borrows();
    // borrow_checker::more_complex_unsafe_borrows();
    borrow_checker::more_complex_unsafe_borrows_miri_success();

    // borrow_checker::basic_array_borrows();
    borrow_checker::more_array_borrows();
    borrow_checker::big_mess_of_array_pointers();

    borrow_checker::array_slice_split_borrow();
    borrow_checker::more_array_slice_borrow();

    borrow_checker::testing_shared_ref_with_raw_pointer();
    borrow_checker::testing_interior_mutability_with_cell();
    borrow_checker::testing_interior_mutability_with_unsafe_cell();

    borrow_checker::testing_box();
    assert_properties();
}
