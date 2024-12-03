fn main() {
    restaurant::front_of_house::hosting::add_to_wait_list();
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::back_of_house::take_care_trash();
    println!("The value of PI is {}", restaurant::PI);
}