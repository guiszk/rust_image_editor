// FINAL PROJECT for Ultimate Rust Crash Course by Nathan Stocks
// source here: https://github.com/CleanCut/ultimate_rust_crash_course/blob/main/exercise/z_final_project/src/main.rs

extern crate clap;
use clap::{Arg, App};

use std::fs;

fn main() {
    //example: cargo run infile.png outfile.png --blur 2.5 --rotate 180 --brighten 100 --invert --fractal
    let matches = App::new("rustimg")
        .version("0.1.0")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use")
            .required(true)
            .index(2))
        .arg(Arg::with_name("blur")
            .long("blur")
            .value_name("NUMBER")
            .help("Sets blur amount")
            .takes_value(true))
        .arg(Arg::with_name("rotate")
            .long("rotate")
            .value_name("DEGREE")
            .help("Rotate image (90, 180 or 270)")
            .takes_value(true))
        .arg(Arg::with_name("brighten")
            .long("brighten")
            .value_name("NUMBER")
            .help("Brighten image")
            .takes_value(true))
        .arg(Arg::with_name("crop")
            .long("crop")
            .value_name("\"x,y,w,h\"")
            .help("Crop image")
            .takes_value(true))
        .arg(Arg::with_name("invert")
            .long("invert")
            .help("Invert image colors"))
        .arg(Arg::with_name("grayscale")
            .long("grayscale")
            .help("Convert to grayscale"))
        .arg(Arg::with_name("solid")
            .long("solid")
            .value_name("color rgb \"252,142,172\"")
            .help("Set image to solid color")
            .takes_value(true))
        .arg(Arg::with_name("fractal")
            .long("fractal")
            .help("Fractalize image"))
    .get_matches();

    //using without unwrap_or because INPUT and OUTPUT are required
    let mut infile = matches.value_of("INPUT").unwrap();
    let outfile = matches.value_of("OUTPUT").unwrap();

    let blur_arg = matches.value_of("blur").unwrap_or("");
    let rotate_arg = matches.value_of("rotate").unwrap_or("");
    let brighten_arg = matches.value_of("brighten").unwrap_or("");
    let crop_arg = matches.value_of("crop").unwrap_or("");
    let invert_arg = matches.is_present("invert");
    let grayscale_arg = matches.is_present("grayscale");
    let solid_arg = matches.value_of("solid").unwrap_or("");
    let fractal_arg = matches.is_present("fractal");

    // get number of valid arguments
    let valid_args_str = [blur_arg, rotate_arg, brighten_arg, crop_arg, solid_arg];
    let valid_args_bool = [invert_arg, grayscale_arg, fractal_arg];
    let mut validnum = 0;

    for i in valid_args_str.iter() {
        if i.to_string() != "" {
            validnum += 1;
        }
    }
    for i in valid_args_bool.iter() {
        if *i {
            validnum += 1;
        }
    }
    
    // if no args, print help
    if validnum < 1 {
        printhelp();
    }
    // if multiple operations, perform all on output image (input image won't keep changes)
    if validnum > 1 {
        fs::copy(infile.to_string(), outfile.to_string()).ok();
        infile = outfile;
    }

    if blur_arg != "" {
        blur(infile.to_string(), outfile.to_string(), blur_arg.parse().expect("Failed to parse a number"));
    }
    if rotate_arg != "" {
        rotate(infile.to_string(), outfile.to_string(), rotate_arg);
    }
    if brighten_arg != "" {
        brighten(infile.to_string(), outfile.to_string(), brighten_arg.parse().expect("Failed to parse a number"));
    }
    if crop_arg != "" {
        crop(infile.to_string(), outfile.to_string(), crop_arg);
    }
    if invert_arg {
        invert(infile.to_string(), outfile.to_string());
    }
    if grayscale_arg {
        grayscale(infile.to_string(), outfile.to_string());
    }
    if solid_arg != "" {
        generate(infile.to_string(), outfile.to_string(), solid_arg);
    }
    if fractal_arg {
        fractal(outfile.to_string());
    }
}

fn printhelp() {
    println!("Usage: cargo run <INPUT> <OUTPUT> [OPTIONS]
    --blur VALUE            gaussian blur 
    --rotate VALUE          rotate image (90, 180 or 270 deg)
    --brighten VALUE        brighten image
    --crop \"x,y,w,h\"        crop image
    --invert                invert colors
    --grayscale             convert to grayscale
    --solid \"r,g,b\"         turn image into solid color
    --fractal               fractalize image
    --help                  show detailed help menu");
    println!("Example: cargo run image.jpg out.jpg --blur 2.5 --invert --rotate 180 --brighten 100");
}

fn blur(infile: String, outfile: String, amount: f32) {
    // open the image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    println!("Blurring image by {}...", amount);
    let img2 = img.blur(amount);
    // save image file
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, amount: &str) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    // parse values
    let img2 = match amount {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => {
            // catch other values
            println!("Invalid value.");
            img
        },
    };
    println!("Rotating image by {}°...", amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    println!("Brightening image by {}...", amount);
    let img2 = img.brighten(amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, amount: &str) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    //x,y,w,h
    let v: Vec<&str> = amount.split(",").collect();
    let x = v[0].parse().expect("Failed to parse a number");
    let y = v[1].parse().expect("Failed to parse a number");
    let w = v[2].parse().expect("Failed to parse a number");
    let h = v[3].parse().expect("Failed to parse a number");
    println!("Cropping image at ({}, {}) with size ({}, {})...", x, y, w, h);
    let img2 = img.crop(x, y, w, h);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    println!("Inverting image...");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    println!("Removing colors...");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(infile: String, outfile: String, col: &str) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let width = img.width();
    let height = img.height();

    // parse r,g,b input
    let v: Vec<&str> = col.split(",").collect();
    let red: u8 = v[0].parse().expect("Failed to parse a number");
    let green: u8 = v[1].parse().expect("Failed to parse a number");
    let blue: u8 = v[2].parse().expect("Failed to parse a number");

    println!("Creating image with color {}...", col);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(outfile).unwrap(); 
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** 
// Make all of the subcommands stackable!
// SOLVED!!