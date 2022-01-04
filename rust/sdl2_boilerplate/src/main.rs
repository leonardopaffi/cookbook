use fermium::{
    prelude::{
        SDL_CreateRenderer, SDL_Delay, SDL_Event, SDL_RenderClear, SDL_RenderPresent,
        SDL_SetRenderDrawColor, SDL_WaitEvent, SDLK_ESCAPE, SDL_KEYDOWN, SDL_KEYUP, SDL_QUIT,
    },
    video::*,
    *,
};

// Tested on Windows with msvc compiler toolchain
pub fn main() {
    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);

        let window = SDL_CreateWindow(
            b"demo\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            800,
            600,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        );
        // Panic if window is not null
        assert!(!window.is_null());

        let renderer = SDL_CreateRenderer(window, -1, 1);
        // Panic if renderer is not null
        assert!(!renderer.is_null());
        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);

        let mut event = SDL_Event::default();

        'gameloop: loop {
            assert_eq!(SDL_WaitEvent(&mut event), 1);
            match event.type_ {
                SDL_QUIT => {
                    println!("Event of type SDL_QUIT has been called. Closing window.");
                    break 'gameloop;
                }
                SDL_KEYDOWN => {
                    // Checking keys
                    match event.key.keysym.sym {
                        SDLK_ESCAPE => {
                            println!("Pressed ESCAPE. Window closed.");
                            break 'gameloop;
                        }
                        _ => (),
                    }
                }
                SDL_KEYUP => {
                    println!("SDL_KEYUP");
                    println!("{:?}", event.key);
                }
                _ => (),
            }

            SDL_RenderClear(renderer);

            SDL_RenderPresent(renderer);

            SDL_Delay(10);
        }

        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
