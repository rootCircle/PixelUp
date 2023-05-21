use pixel_up::Args;

fn main() {
    let mut args = Args::new();

    let img_result = args.check_and_open();

    let mut img_result = img_result.unwrap();

    args.apply_filters(&mut img_result);
}
