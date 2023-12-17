use std::vec;

mod utils;
mod y2023;

fn main() {
    clumsy_crucible();
    floor_is_lava();
    lens_library();
    parabolic_reflector_dish();
    point_of_incidence();
    hot_springs();
    cosmic_expansion();
    pipe_maze();
    mirage_maintenance();
    haunted_wasteland();
    camel_cards();
    wait_for_it();
    fertilizer();
    scratchcards();
    gear_ratios();
    cube_conundrum();
    trebuchet();
}

fn trebuchet() {
    y2023::d01::trebuchet::trebuchet_part_one("src/y2023/d01/example_01.txt");
    y2023::d01::trebuchet::trebuchet_part_one("src/y2023/d01/input.txt");
    y2023::d01::trebuchet::trebuchet_part_two("src/y2023/d01/example_02.txt");
    y2023::d01::trebuchet::trebuchet_part_two("src/y2023/d01/input.txt");
}

fn cube_conundrum() {
    y2023::d02::cube_conundrum::cube_conundrum_part_one("src/y2023/d02/example.txt");
    y2023::d02::cube_conundrum::cube_conundrum_part_one("src/y2023/d02/input.txt");
    y2023::d02::cube_conundrum::cube_conundrum_part_two("src/y2023/d02/example.txt");
    y2023::d02::cube_conundrum::cube_conundrum_part_two("src/y2023/d02/input.txt");
}

fn gear_ratios() {
    y2023::d03::gear_ratios::gear_ratios_part_one("src/y2023/d03/example.txt");
    y2023::d03::gear_ratios::gear_ratios_part_one("src/y2023/d03/input.txt");
    y2023::d03::gear_ratios::gear_ratios_part_two("src/y2023/d03/example.txt");
    y2023::d03::gear_ratios::gear_ratios_part_two("src/y2023/d03/input.txt");
}

fn scratchcards() {
    y2023::d04::scratchcards::scratchcards_part_one("src/y2023/d04/example.txt");
    y2023::d04::scratchcards::scratchcards_part_one("src/y2023/d04/input.txt");
    y2023::d04::scratchcards::scratchcards_part_two("src/y2023/d04/example.txt");
    y2023::d04::scratchcards::scratchcards_part_two("src/y2023/d04/input.txt");
}

fn fertilizer() {
    y2023::d05::fertilizer::fertilizer_part_one("src/y2023/d05/example.txt");
    y2023::d05::fertilizer::fertilizer_part_one("src/y2023/d05/input.txt");
    y2023::d05::fertilizer::fertilizer_part_two("src/y2023/d05/example.txt");
    // y2023::d05::fertilizer::fertilizer_part_two("src/y2023/d05/input.txt");
}

fn wait_for_it() {
    y2023::d06::wait_for_it::wait_for_it(vec![7, 15, 30], vec![9, 40, 200]);
    y2023::d06::wait_for_it::wait_for_it(vec![56, 71, 79, 99], vec![334, 1135, 1350, 2430]);
    y2023::d06::wait_for_it::wait_for_it(vec![71530], vec![940200]);
    y2023::d06::wait_for_it::wait_for_it(vec![56717999], vec![334113513502430]);
}

fn camel_cards() {
    y2023::d07::camel_cards::camel_cards_part_one("src/y2023/d07/example.txt");
    y2023::d07::camel_cards::camel_cards_part_one("src/y2023/d07/input.txt");
    y2023::d07::camel_cards::camel_cards_part_two("src/y2023/d07/example.txt");
    y2023::d07::camel_cards::camel_cards_part_two("src/y2023/d07/input.txt");
}

fn haunted_wasteland() {
    y2023::d08::haunted_wasteland::haunted_wasteland_part_one("src/y2023/d08/example_01.txt");
    y2023::d08::haunted_wasteland::haunted_wasteland_part_one("src/y2023/d08/example_02.txt");
    y2023::d08::haunted_wasteland::haunted_wasteland_part_one("src/y2023/d08/input.txt");
    y2023::d08::haunted_wasteland::haunted_wasteland_part_two("src/y2023/d08/example_03.txt");
    y2023::d08::haunted_wasteland::haunted_wasteland_part_two("src/y2023/d08/input.txt");
}

fn mirage_maintenance() {
    y2023::d09::mirage_maintenance::mirage_maintenance_part_one("src/y2023/d09/example.txt");
    y2023::d09::mirage_maintenance::mirage_maintenance_part_one("src/y2023/d09/input.txt");
    y2023::d09::mirage_maintenance::mirage_maintenance_part_two("src/y2023/d09/example.txt");
    y2023::d09::mirage_maintenance::mirage_maintenance_part_two("src/y2023/d09/input.txt");
}

fn pipe_maze() {
    y2023::d10::pipe_maze::pipe_maze_part_one("src/y2023/d10/example_01.txt");
    y2023::d10::pipe_maze::pipe_maze_part_one("src/y2023/d10/example_02.txt");
    y2023::d10::pipe_maze::pipe_maze_part_one("src/y2023/d10/input.txt");
    y2023::d10::pipe_maze::pipe_maze_part_two("src/y2023/d10/input.txt");
}

fn cosmic_expansion() {
    y2023::d11::cosmic_expansion::cosmic_expansion("src/y2023/d11/example.txt", 1);
    // y2023::d11::cosmic_expansion::cosmic_expansion("src/y2023/d11/input.txt", 1);
    y2023::d11::cosmic_expansion::cosmic_expansion("src/y2023/d11/example.txt", 9);
    y2023::d11::cosmic_expansion::cosmic_expansion("src/y2023/d11/example.txt", 99);
    // y2023::d11::cosmic_expansion::cosmic_expansion("src/y2023/d11/input.txt", 999_999);
}

fn hot_springs() {
    y2023::d12::hot_springs::hot_springs_part_one("src/y2023/d12/example.txt");
    y2023::d12::hot_springs::hot_springs_part_one("src/y2023/d12/input.txt");
    y2023::d12::hot_springs::hot_springs_part_two("src/y2023/d12/example.txt");
    y2023::d12::hot_springs::hot_springs_part_two("src/y2023/d12/input.txt");
}

fn point_of_incidence() {
    y2023::d13::point_of_incidence::point_of_incidence_part_one("src/y2023/d13/example.txt");
    y2023::d13::point_of_incidence::point_of_incidence_part_one("src/y2023/d13/input.txt");
    y2023::d13::point_of_incidence::point_of_incidence_part_two("src/y2023/d13/example.txt");
    y2023::d13::point_of_incidence::point_of_incidence_part_two("src/y2023/d13/input.txt");
}

fn parabolic_reflector_dish() {
    y2023::d14::parabolic_reflector_dish::parabolic_reflector_dish_part_one("src/y2023/d14/example.txt");
    y2023::d14::parabolic_reflector_dish::parabolic_reflector_dish_part_one("src/y2023/d14/input.txt");
    y2023::d14::parabolic_reflector_dish::parabolic_reflector_dish_part_two("src/y2023/d14/example.txt");
    y2023::d14::parabolic_reflector_dish::parabolic_reflector_dish_part_two("src/y2023/d14/input.txt");
}

fn lens_library() {
    y2023::d15::lens_library::lens_library_part_one("src/y2023/d15/example.txt");
    y2023::d15::lens_library::lens_library_part_one("src/y2023/d15/input.txt");
    y2023::d15::lens_library::lens_library_part_two("src/y2023/d15/example.txt");
    y2023::d15::lens_library::lens_library_part_two("src/y2023/d15/input.txt");
}

fn floor_is_lava() {
    y2023::d16::floor_is_lava::floor_is_lava_part_one("src/y2023/d16/example.txt");
    // y2023::d16::floor_is_lava::floor_is_lava_part_one("src/y2023/d16/input.txt");
    y2023::d16::floor_is_lava::floor_is_lava_part_two("src/y2023/d16/example.txt");
    // y2023::d16::floor_is_lava::floor_is_lava_part_two("src/y2023/d16/input.txt");
}

fn clumsy_crucible() {
    y2023::d17::clumsy_crucible::clumsy_crucible_part_one("src/y2023/d17/example.txt");
    y2023::d17::clumsy_crucible::clumsy_crucible_part_one("src/y2023/d17/input.txt");
    y2023::d17::clumsy_crucible::clumsy_crucible_part_two("src/y2023/d17/example.txt");
    y2023::d17::clumsy_crucible::clumsy_crucible_part_two("src/y2023/d17/input.txt");
}