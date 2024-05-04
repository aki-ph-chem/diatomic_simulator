extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;
    let ui = include_str!("ui/ui_mw.ui");
    let builder = gtk::Builder::from_string(ui);

    // generate window
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::glib::Propagation::Stop
    });

    // quit program by "Quit" button
    let quit: gtk::MenuItem = builder.object("quit").expect("Error: quit");
    quit.connect_activate(move |_| {
        gtk::main_quit();
    });
    // quit program by Ctrl + Q
    let accel_group = gtk::AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator(
        "activate",
        &accel_group,
        key,
        modifier,
        gtk::AccelFlags::VISIBLE,
    );

    // const menu
    let const_menu: gtk::MenuItem = builder.object("const").expect("Error: const");

    // generate About dialog by click "About" button
    let about: gtk::MenuItem = builder.object("about").expect("Error: about");
    let about_dialog: gtk::AboutDialog =
        builder.object("about_dialog").expect("Error: about_dialog");
    let window_ = window.clone();
    about.connect_activate(move |_| {
        about_dialog.set_title("About");
        about_dialog.set_authors(&["Aki"]);
        about_dialog.set_transient_for(Some(&window_));
        about_dialog.run();
        about_dialog.hide();
    });

    // show window & enter event loop
    window.show_all();
    gtk::main();

    Ok(())
}
