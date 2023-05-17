use pixel_up::Args;


fn main() {
    let mut args = Args::new();
    
    let img_result = args.check_and_open();
    
    let img_result = img_result.unwrap();

    args.apply_filters(img_result);
    
}