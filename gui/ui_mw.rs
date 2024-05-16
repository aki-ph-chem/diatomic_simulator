use diatomic_simulator::{
    microwave::Population,
    utl::{convolute_lorentz, LineShape},
};
use gtk;
use gtk::glib;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct PlotRange {
    pub x_min: f64,
    pub x_max: f64,
}

impl PlotRange {
    pub fn new(x_min: f64, x_max: f64) -> Self {
        Self { x_min, x_max }
    }
}

fn generate_entry(builder: &gtk::Builder, id: &str, initial_value: &str) -> gtk::Entry {
    let error_message = format!("Error: {id}");
    let entry: gtk::Entry = builder.object(id).expect(&error_message);
    entry.set_text(initial_value);

    entry
}

fn plot_spectrum(
    backend: CairoBackend,
    spectrum: Rc<RefCell<Population>>,
    lorentz_line_shape: Rc<RefCell<LineShape>>,
    plot_range: Rc<RefCell<PlotRange>>,
) -> Result<(), Box<dyn Error>> {
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Spectrum", ("snas-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            plot_range.borrow().x_min..plot_range.borrow().x_max,
            0.0..1.2,
        )?;
    chart.configure_mesh().draw()?;

    let (signal_raw_x, signal_raw_y) = spectrum.borrow().calc_spectrum();
    let (signal_x, mut signal_y) = convolute_lorentz(
        plot_range.borrow().x_min,
        plot_range.borrow().x_max,
        0.01,
        &lorentz_line_shape.borrow(),
        (&signal_raw_x, &signal_raw_y),
    );

    // normalize spectrum
    let mut max_signal_y = 0.0_f64;
    for v in &signal_y {
        max_signal_y = max_signal_y.max(*v);
    }
    for v in &mut signal_y {
        *v /= max_signal_y;
    }

    chart.draw_series(LineSeries::new(
        signal_x.iter().zip(signal_y.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?;
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;
    let ui = include_str!("ui/ui_mw.ui");
    let builder = gtk::Builder::from_string(ui);

    // generate window
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
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

    // init state
    let spectrum = Rc::new(RefCell::new(Population::new(300.0, 30, 1200.0, 2.0)));
    let lorentz_line_shape = Rc::new(RefCell::new(LineShape::new(0.004)));
    let plot_range = Rc::new(RefCell::new(PlotRange::new(1000.0, 1400.0)));
    // init entrys
    let (
        entry_temperature,
        entry_lorentz_width,
        entry_band_origin,
        entry_j_max,
        entry_rot_const,
        entry_x_min,
        entry_x_max,
    ) = (
        generate_entry(
            &builder,
            "entry_temperature",
            &spectrum.borrow().temperature.to_string(),
        ),
        generate_entry(
            &builder,
            "entry_lorentz_width",
            &lorentz_line_shape.borrow().width_lorentz.to_string(),
        ),
        generate_entry(
            &builder,
            "entry_band_origin",
            &spectrum.borrow().band_origin.to_string(),
        ),
        generate_entry(
            &builder,
            "entry_j_max",
            &spectrum.borrow().j_max.to_string(),
        ),
        generate_entry(
            &builder,
            "entry_rot_const",
            &spectrum.borrow().rot_const().to_string(),
        ),
        generate_entry(
            &builder,
            "entry_x_min",
            &plot_range.borrow().x_min.to_string(),
        ),
        generate_entry(
            &builder,
            "entry_x_max",
            &plot_range.borrow().x_max.to_string(),
        ),
    );

    // init redraw button
    let button_redraw: gtk::Button = builder
        .object("button_redraw")
        .expect("Error: button_redraw");
    // init plot area
    let plot_area: gtk::DrawingArea = builder.object("plot_area").expect("Error: plot_area");

    // initial plot
    let (spectrum_clone, lorentz_line_shape_clone, plot_range_clone) = (
        spectrum.clone(),
        lorentz_line_shape.clone(),
        plot_range.clone(),
    );
    plot_area.connect_draw(move |widget, cr| {
        let (width, height) = (
            widget.allocated_width() as u32,
            widget.allocated_height() as u32,
        );
        let back_end = CairoBackend::new(cr, (width, height)).expect("Error: init backend");
        plot_spectrum(
            back_end,
            spectrum_clone.clone(),
            lorentz_line_shape_clone.clone(),
            plot_range_clone.clone(),
        )
        .expect("Error: plot_spectrum");

        Inhibit(false)
    });

    // redraw by changed temperature, bnad_origin, rot_const
    let handle_parameters_update =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut Population) -> &mut f64 + 'static>| {
            button_redraw.connect_clicked(
                glib::clone!(@weak control, @weak plot_area, @weak spectrum => move |_| {
                let mut state = spectrum.borrow_mut();
                match control.text().parse::<f64>() {
                    Ok(value) => {
                        *action(&mut *state) = value;
                        plot_area.queue_draw();
                    },
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                }),
            );
        };
    handle_parameters_update(&entry_temperature, Box::new(|spr| &mut spr.temperature));
    handle_parameters_update(&entry_band_origin, Box::new(|spr| &mut spr.band_origin));
    handle_parameters_update(&entry_rot_const, Box::new(|spr| spr.rot_const_ref()));

    let handle_j_max_update =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut Population) -> &mut i32 + 'static>| {
            button_redraw.connect_clicked(
                glib::clone!(@weak control, @weak plot_area, @weak spectrum => move |_| {
                let mut state = spectrum.borrow_mut();
                match control.text().parse::<i32>() {
                    Ok(value) => {
                        *action(&mut *state) = value;
                        plot_area.queue_draw();
                    },
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                }),
            );
        };
    handle_j_max_update(&entry_j_max, Box::new(|spr| &mut spr.j_max));

    let handle_width_update =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut LineShape) -> &mut f64 + 'static>| {
            button_redraw.connect_clicked(
                glib::clone!(@weak control, @weak plot_area, @weak lorentz_line_shape  => move |_| {
                let mut state = lorentz_line_shape.borrow_mut();
                match control.text().parse::<f64>() {
                    Ok(value) => {
                        *action(&mut *state) = value;
                        plot_area.queue_draw();
                    },
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                }),
            );
        };
    handle_width_update(
        &entry_lorentz_width,
        Box::new(|line_shape| &mut line_shape.width_lorentz),
    );

    let handle_plot_range_update =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut PlotRange) -> &mut f64 + 'static>| {
            button_redraw.connect_clicked(
                glib::clone!(@weak control, @weak plot_area, @weak plot_range  => move |_| {
                let mut state = plot_range.borrow_mut();
                match control.text().parse::<f64>() {
                    Ok(value) => {
                        *action(&mut *state) = value;
                        plot_area.queue_draw();
                    },
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                }),
            );
        };
    handle_plot_range_update(&entry_x_min, Box::new(|range| &mut range.x_min));
    handle_plot_range_update(&entry_x_max, Box::new(|range| &mut range.x_max));

    // show window & enter event loop
    window.show_all();
    gtk::main();

    Ok(())
}
