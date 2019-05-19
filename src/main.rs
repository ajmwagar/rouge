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

fn main() {
    // Init the root window here. All other settings fallback to default
    let root = Root::initializer()
        .font(
            // "./fonts/DarkondDigsDeeper_16x16.png",
            "./fonts/Cheepicus_8x8x2.png",
            FontLayout::AsciiInRow,
        )
        .font_type(FontType::Default)
        .font_dimensions(16, 16)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rouge")
        .init();

    // Limit FPS here
    tcod::system::set_fps(LIMIT_FPS);

    // Create the Tcod instance
    let mut tcod = Tcod {
        root: root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        mouse: Default::default(),
    };

    main_menu(&mut tcod);
}
