// Create a CLI tool: random image with n randomly colored pixels, where n is the input.
//
// The tool should take the following arguments:
// - width
// - height
// - output
//
// The tool should generate a random image with the given width and height and save it to the given output file.
//
// The tool should use clap to parse the arguments.
//
// The tool should use rand to generate the random pixels.
//
// The tool should use image to save the image.
//
// The tool should use the following code to generate the image:
//
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "A random image generator."
)]
struct Cli {
    #[clap(short, long)]
    width: u32,
    #[clap(short, long)]
    height: u32,
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Cli::parse();
    let imgx = args.width;
    let imgy = args.height;
    let output = args.output;
    
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    println!("{args:?}");

   
   /* // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut rng = rand::thread_rng();
    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = rng.gen_range(0..255) as u8;
        let g = rng.gen_range(0..255) as u8;
        let b = rng.gen_range(0..255) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(output + ".png").unwrap();
    */
}
