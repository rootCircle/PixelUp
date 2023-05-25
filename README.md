# 🖼️ PixelUp

PixelUp is a Rust-based CLI tool that allows users to apply image filters offline while
 ensuring their privacy. With PixelUp, you can apply filters like sepia, grayscale, negative
, blur, reflection, and edge to your images without worrying about any data being stored or
 sent over the internet.

## Usage

To use PixelUp, you'll need to have Rust installed on your machine. Once you have Rust
 installed, you can clone the project and then install PixelUp by running cargo install pixel_up.

### Options

Here are the available options for PixelUp:

- -i, --src <SRC>: The path to the source image you want to apply filters to.
- -o, --output <OUTPUT>: The path where you want to save the filtered image. If not
 provided, the filtered image will be saved in the same directory in png format with output.png filename.
- -f, --filter <FILTER>: The filter you want to apply to the image. Possible values are 
sepia, grayscale, negative, blur, reflection, and edge.
- -h, --help: Print help.
- -V, --version: Print version.

### Example

Here's an example command to apply the sepia filter to an image:

```
pixel_up -i /path/to/image.jpg -f sepia
```

This will create a new image with the sepia filter applied in the same directory as the
 original image.

## Contribution

If you encounter any issues while using PixelUp or have suggestions for improvement, feel
 free to submit an issue or pull request on the Github repository.

## License

PixelUp is licensed under the Apache 2.0 License. See the [LICENSE](https://github.com/rootCircle/PixelUp/blob/main/LICENSE) file for more information.