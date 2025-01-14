#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::criterion_main;

mod mesh_all;

criterion_main! {
    mesh_all::empty::mesh_two::mesh_two,
    // mesh_all::empty::mesh_three::mesh_three,
    // mesh_all::empty::mesh_four::mesh_four,
    // mesh_all::empty::mesh_five::mesh_five,
    // mesh_all::empty::mesh_six::mesh_six,
    // mesh_all::empty::mesh_seven::mesh_seven,
    // mesh_all::empty::mesh_eight::mesh_eight,
    // mesh_all::empty::mesh_nine::mesh_nine,
    // mesh_all::empty::mesh_ten::mesh_ten,
    // mesh_all::empty::mesh_eleven::mesh_eleven,
    // mesh_all::empty::mesh_twenty::mesh_twenty,
    // //////////
    // mesh_all::normal::mesh_two::mesh_two,
    // mesh_all::normal::mesh_three::mesh_three,
    // mesh_all::normal::mesh_four::mesh_four,
    // mesh_all::normal::mesh_five::mesh_five,
    // mesh_all::normal::mesh_six::mesh_six,
    // mesh_all::normal::mesh_seven::mesh_seven,
    // mesh_all::normal::mesh_eight::mesh_eight,
    // mesh_all::normal::mesh_nine::mesh_nine,
    // mesh_all::normal::mesh_ten::mesh_ten,
    // mesh_all::normal::mesh_eleven::mesh_eleven,
    // mesh_all::normal::mesh_twenty::mesh_twenty,
    // //////////
    // mesh_all::cancel_broadcast::mesh_three::mesh_three,
    // mesh_all::cancel_broadcast::mesh_four::mesh_four,
    // mesh_all::cancel_broadcast::mesh_five::mesh_five,
    // mesh_all::cancel_broadcast::mesh_six::mesh_six,
    // mesh_all::cancel_broadcast::mesh_seven::mesh_seven,
    // mesh_all::cancel_broadcast::mesh_eight::mesh_eight,
    // mesh_all::cancel_broadcast::mesh_nine::mesh_nine,
    // mesh_all::cancel_broadcast::mesh_ten::mesh_ten,
    // mesh_all::cancel_broadcast::mesh_eleven::mesh_eleven,
    // mesh_all::cancel_broadcast::mesh_twenty::mesh_twenty,
    // //////////
    // mesh_all::cancel::mesh_two::mesh_two,
    // mesh_all::cancel::mesh_three::mesh_three,
    // mesh_all::cancel::mesh_four::mesh_four,
    // mesh_all::cancel::mesh_five::mesh_five,
    // mesh_all::cancel::mesh_six::mesh_six,
    // mesh_all::cancel::mesh_seven::mesh_seven,
    // mesh_all::cancel::mesh_eight::mesh_eight,
    // mesh_all::cancel::mesh_nine::mesh_nine,
    // mesh_all::cancel::mesh_ten::mesh_ten,
    // mesh_all::cancel::mesh_eleven::mesh_eleven,
    // mesh_all::cancel::mesh_twenty::mesh_twenty,
    // //////////
    // mesh_all::baking_cancel::mesh_two::mesh_two,
    // mesh_all::baking_cancel::mesh_three::mesh_three,
    // mesh_all::baking_cancel::mesh_four::mesh_four,
    // mesh_all::baking_cancel::mesh_five::mesh_five,
    // mesh_all::baking_cancel::mesh_six::mesh_six,
    // mesh_all::baking_cancel::mesh_seven::mesh_seven,
    // mesh_all::baking_cancel::mesh_eight::mesh_eight,
    // mesh_all::baking_cancel::mesh_nine::mesh_nine,
    // mesh_all::baking_cancel::mesh_ten::mesh_ten,
    // mesh_all::baking_cancel::mesh_eleven::mesh_eleven,
    // mesh_all::baking_cancel::mesh_twenty::mesh_twenty,
    // //////////
    // mesh_all::baking_cancel_inline::mesh_two::mesh_two,
    // mesh_all::baking_cancel_inline::mesh_three::mesh_three,
    // mesh_all::baking_cancel_inline::mesh_four::mesh_four,
    // mesh_all::baking_cancel_inline::mesh_five::mesh_five,
    // mesh_all::baking_cancel_inline::mesh_six::mesh_six,
    // mesh_all::baking_cancel_inline::mesh_seven::mesh_seven,
    // mesh_all::baking_cancel_inline::mesh_eight::mesh_eight,
    // mesh_all::baking_cancel_inline::mesh_nine::mesh_nine,
    // mesh_all::baking_cancel_inline::mesh_ten::mesh_ten,
    // mesh_all::baking_cancel_inline::mesh_eleven::mesh_eleven,
    // mesh_all::baking_cancel_inline::mesh_twenty::mesh_twenty,
}
