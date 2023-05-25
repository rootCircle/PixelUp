use pixel_up::Args;

fn main() {
    let mut args = Args::new();

    let mut img_result = args.check_and_open().unwrap();

    args.apply_filters(&mut img_result);
}
