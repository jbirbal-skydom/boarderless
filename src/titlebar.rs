//use slint::Weak;

slint::include_modules!();

fn main() {
    // slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));
    let main_window = CustomWindow::new().unwrap();
    // let main_window_weak = main_window.as_weak();

    main_window.on_close(move || {
        //todo
        unimplemented!("code to close the window")

    });

    main_window.on_minimize(move || {
        //todo
        unimplemented!("minimize the window")
    });

    main_window.on_move(move |x, y| {
        // todo
        unimplemented!("move the window to x: {}, y: {}", x, y)

    });

    main_window.on_resize(move |x, y| {
        // todo
        unimplemented!("resize: {}, y: {}", x, y)

    });


    main_window.run().unwrap();
}