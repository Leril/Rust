mod front_of_house;

mod front_of_the_house{
    mod serving{
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    front_of_the_house::hosting::add_to_wait_list();
}