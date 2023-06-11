use clap::Parser;
use image2textart;

use std::process::exit;

const TITLE: &str = r"
_____                            ___  _            _             _   
|_   _|                          |__ \| |          | |           | |  
  | |  _ __ ___   __ _  __ _  ___   ) | |_ _____  _| |_ __ _ _ __| |_ 
  | | | '_ ` _ \ / _` |/ _` |/ _ \ / /| __/ _ \ \/ | __/ _` | '__| __|
 _| |_| | | | | | (_| | (_| |  __// /_| ||  __/>  <| || (_| | |  | |_ 
|_____|_| |_| |_|\__,_|\__, |\___|____|\__\___/_/\_\\__\__,_|_|   \__|
                        __/ |                                         
                       |___/        

Text art generator
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = TITLE)]
struct Args {
    #[arg(help = "Path to the image file.")]
    filename: String,
    #[arg(
        short = 'W',
        long = "width",
        default_value = None,
        help = "Width of the output image"
    )]
    width: Option<u32>,
    #[arg(
        short = 'H',
        long = "height",
        default_value = None,
        help = "Height of the output image."
    )]
    height: Option<u32>,
    #[arg(
        short = 'r',
        long = "ratio",
        default_value_t = 1.0,
        help = "Specifies the scale of the image."
    )]
    ratio: f32,
    #[arg(
        short = 'c',
        long = "color",
        default_value = "gray",
        help = "What colors should be used in the output image."
    )]
    color: String,
    #[arg(
        short = 'C',
        long = "charset",
        default_value = "default",
        help = "Symbols used to render the output image, from translucent to opaque. Built-in charsets: ansi, default, slight."
    )]
    charset: String,
    #[arg(
        long = "custom-charset",
        default_value = None,
        help = "Custom symbols used to render the output image."
    )]
    custom_charset: Option<String>,
    #[arg(
        short = 'i',
        long = "invert",
        default_value_t = false,
        help = "Inverts the weights of the symbols. Useful for white backgrounds."
    )]
    invert: bool,
}

fn get_charset_by_name(charset_name: &str, custom_chartset: Option<String>) -> String {
    match charset_name {
        "ansi" => String::from(" ░▒▓█"),
        "default" => String::from(
            "  .`^\"\\,:;Il!i><~+_-?][}{1)(|\\\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B$@",
        ),
        "slight" => String::from("   .`\"\\:I!>~_?[{)|\\\\YLpda*W8%@$"),
        _ => {
            if let Some(_cc) = custom_chartset {
                return _cc;
            }
            eprintln!(
                "Such charset: '{charset}' not found",
                charset = charset_name
            );
            exit(1)
        }
    }
}

fn get_color_by_name(color_name: &str) -> image2textart::Color {
    match color_name {
        "full" => image2textart::Color::Full,
        "char" => image2textart::Color::Char,
        "background" => image2textart::Color::Background,
        "gray" => image2textart::Color::Gray,
        "no-color" => image2textart::Color::NoColor,
        _ => {
            eprintln!("Such color: '{color}' not found", color = color_name);
            exit(1)
        }
    }
}

fn main() {
    let args = Args::parse();

    let img = match image::open(args.filename) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    let options = image2textart::Options {
        width: args.width,
        height: args.height,
        color: get_color_by_name(&args.color),
        charset: get_charset_by_name(&args.charset, args.custom_charset),
        ratio: args.ratio,
        invert: args.invert,
    };

    let text_art_matrix = image2textart::image2text_art_matrix(img, options);

    print!("{}", text_art_matrix);
}
