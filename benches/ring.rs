use criterion::criterion_main;

mod ring_all;

criterion_main! {
    ring_all::empty::ring_two::ring_two,
    // ring_all::empty::ring_three::ring_three,
    // ring_all::empty::ring_four::ring_four,
    // ring_all::empty::ring_five::ring_five,
    // ring_all::empty::ring_six::ring_six,
    // ring_all::empty::ring_seven::ring_seven,
    // ring_all::empty::ring_eight::ring_eight,
    // ring_all::empty::ring_nine::ring_nine,
    // ring_all::empty::ring_ten::ring_ten,
    // ring_all::empty::ring_eleven::ring_eleven,
    // ring_all::empty::ring_twenty::ring_twenty,
    // //////////
    // ring_all::normal::ring_two::ring_two,
    // ring_all::normal::ring_three::ring_three,
    // ring_all::normal::ring_four::ring_four,
    // ring_all::normal::ring_five::ring_five,
    // ring_all::normal::ring_six::ring_six,
    // ring_all::normal::ring_seven::ring_seven,
    // ring_all::normal::ring_eight::ring_eight,
    // ring_all::normal::ring_nine::ring_nine,
    // ring_all::normal::ring_ten::ring_ten,
    // ring_all::normal::ring_eleven::ring_eleven,
    // ring_all::normal::ring_twenty::ring_twenty,
    // //////////
    // ring_all::cancel_broadcast::ring_three::ring_three,
    // ring_all::cancel_broadcast::ring_four::ring_four,
    // ring_all::cancel_broadcast::ring_five::ring_five,
    // ring_all::cancel_broadcast::ring_six::ring_six,
    // ring_all::cancel_broadcast::ring_seven::ring_seven,
    // ring_all::cancel_broadcast::ring_eight::ring_eight,
    // ring_all::cancel_broadcast::ring_nine::ring_nine,
    // ring_all::cancel_broadcast::ring_ten::ring_ten,
    // ring_all::cancel_broadcast::ring_eleven::ring_eleven,
    // ring_all::cancel_broadcast::ring_twenty::ring_twenty,
    // //////////
    // ring_all::cancel::ring_two::ring_two,
    // ring_all::cancel::ring_three::ring_three,
    // ring_all::cancel::ring_four::ring_four,
    // ring_all::cancel::ring_five::ring_five,
    // ring_all::cancel::ring_six::ring_six,
    // ring_all::cancel::ring_seven::ring_seven,
    // ring_all::cancel::ring_eight::ring_eight,
    // ring_all::cancel::ring_nine::ring_nine,
    // ring_all::cancel::ring_ten::ring_ten,
    // ring_all::cancel::ring_eleven::ring_eleven,
    // ring_all::cancel::ring_twenty::ring_twenty,
    // //////////
    // ring_all::baking_cancel::ring_two::ring_two,
    // ring_all::baking_cancel::ring_three::ring_three,
    // ring_all::baking_cancel::ring_four::ring_four,
    // ring_all::baking_cancel::ring_five::ring_five,
    // ring_all::baking_cancel::ring_six::ring_six,
    // ring_all::baking_cancel::ring_seven::ring_seven,
    // ring_all::baking_cancel::ring_eight::ring_eight,
    // ring_all::baking_cancel::ring_nine::ring_nine,
    // ring_all::baking_cancel::ring_ten::ring_ten,
    // ring_all::baking_cancel::ring_eleven::ring_eleven,
    // ring_all::baking_cancel::ring_twenty::ring_twenty,
    // //////////
    // ring_all::baking_cancel_inline::ring_two::ring_two,
    // ring_all::baking_cancel_inline::ring_three::ring_three,
    // ring_all::baking_cancel_inline::ring_four::ring_four,
    // ring_all::baking_cancel_inline::ring_five::ring_five,
    // ring_all::baking_cancel_inline::ring_six::ring_six,
    // ring_all::baking_cancel_inline::ring_seven::ring_seven,
    // ring_all::baking_cancel_inline::ring_eight::ring_eight,
    // ring_all::baking_cancel_inline::ring_nine::ring_nine,
    // ring_all::baking_cancel_inline::ring_ten::ring_ten,
    // ring_all::baking_cancel_inline::ring_eleven::ring_eleven,
    // ring_all::baking_cancel_inline::ring_twenty::ring_twenty,
}
