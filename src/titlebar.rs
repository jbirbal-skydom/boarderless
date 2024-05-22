//use slint::Weak;
use i_slint_backend_winit::WinitWindowAccessor;

slint::include_modules!();

fn main() {
    // slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));
    let main_window = CustomTitlebar::new().unwrap();
    let main_window_weak = main_window.as_weak();

    main_window.on_close(move || {
        //todo
        unimplemented!("code to close the window")

    });

    main_window.on_minimize(move || {
        //todo
        unimplemented!("minimize the window")
    });

    main_window.on_move(move || {
        // todo
        // unimplemented!("move the window to x: {}, y: {}", x, y)
        let app_clone = main_window_weak.unwrap();
        app_clone.window().with_winit_window(|win| {
            let _ = win.drag_window();
        });

    });

    main_window.on_resize(move |x, y| {
        // todo
        unimplemented!("resize: {}, y: {}", x, y)

    });


    main_window.run().unwrap();
}