use std::vec;
mod utils;
mod y2023;

fn main() {
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