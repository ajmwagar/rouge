/*
 * CRATES/USE calls
 */

use tcod::console::*;
use tcod::map::Map as FovMap;

use rouge::func::*;
use rouge::r#const::*;
use rouge::types::*;

/*
 * Finally we're at main. This includes our map/player generation and gameloop.
 */

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

/// A small procedural roguelike 
#[derive(StructOpt, Debug)]
#[structopt(name = "rouge")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate fullscreen mode
    #[structopt(long = "fullscreen")]
    fullscreen: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Set max fps
    #[structopt(short = "s", long = "fps", default_value = "60")]
    fps: i32,

    /// Font file
    #[structopt(short = "f", long = "font", default_value = "./fonts/Cheepicus_8x8x2.png", parse(from_os_str))]
    font: PathBuf,

    // /// Number of cars
    // #[structopt(short = "c", long = "nb-cars")]
    // nb_cars: Option<i32>,

    // /// admin_level to consider
    // #[structopt(short = "l", long = "level")]
    // level: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    // Limit FPS here
    tcod::system::set_fps(opt.fps);

    // Init the root window here. All other settings fallback to default
    let root = Root::initializer()
	.font(
	    opt.font,
	    // "./fonts/Phoebus_16x16.png",
            // "./fonts/Hermano.png",
	    FontLayout::AsciiInRow,
	    )
	.font_type(FontType::Default)
	.font_dimensions(16, 16)
        .renderer(Renderer::GLSL)
	.size(SCREEN_WIDTH, SCREEN_HEIGHT)
	.title(GAME_TITLE)
	.fullscreen(opt.fullscreen)
	.init();


    println!("SDL init");

    // Create the Tcod instance
    let mut tcod = Tcod {
	root: root,
	con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
	panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
	fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
	mouse: Default::default(),
    };

    use std::fs::File;
    use std::io::BufReader;
    use rodio::Source;

    let device = rodio::default_output_device().unwrap();

    let sink = rodio::Sink::new(&device);

    let file = File::open("./assets/soundtrack.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();

    sink.append(source);

    println!("Main Menu init");
    main_menu(&mut tcod);
}
