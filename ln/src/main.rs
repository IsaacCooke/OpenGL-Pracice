use core::panic;

extern crate sdl2;

fn main() {
    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(err) => panic!("SDL could not be initialised! SDL_Error: {}", err),
    };

    let video = match sdl_context.video() {
        Ok(video) => video,
        Err(err) => panic!("Could not get video subsystem SDL_Error: {}", err),
    };
}
