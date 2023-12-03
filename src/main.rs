mod utils;
mod y2023;

fn main() {
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