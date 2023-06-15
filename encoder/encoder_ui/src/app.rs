use eframe::{
    egui::{self, ComboBox, Context, Layout},
    emath::Align,
};
use egui_extras::Size;
use encoder_lib::{command::Command, encoder::Encoder};
use logger::logger::Logger;

use crate::error::Error;

macro_rules! btn_sized {
    ($ui:expr, $w:expr, $h:expr, $lbl:expr) => {
        $ui.add_sized(
            [$ui.available_width() - $w, $ui.available_height() - $h],
            egui::Button::new($lbl),
        )
    };
}
macro_rules! control {
    ($cmd:expr, $ui:expr, $slf:expr) => {{
        let cmd = $cmd;
        if btn_sized!($ui, 5., 5., cmd.to_string()).clicked() {
            let _res = $slf.encoder.add_step(cmd);
        }
    }};
}

pub(crate) struct App {
    // pub(crate) toasts: Toasts,
    port: Option<String>,
    encoder: Encoder,
    err: Option<Error>,
    logger: Logger,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("{:?}", egui::Visuals::default());
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .iter_mut()
            .for_each(|font| font.1.tweak.scale = 1.3);

        cc.egui_ctx.set_fonts(fonts);

        Self {
            // toasts: Toasts::default(),
            port: None,
            encoder: Encoder::default(),
            err: None,
            logger: Logger::default(),
        }
    }

    fn top_menu(&mut self, ui: &mut egui::Ui) {
        let main_grid = egui_grid::GridBuilder::new()
            .new_row(Size::relative(1.))
            .cell(Size::relative(0.5))
            .with_layout(Layout::left_to_right(Align::Center))
            .nest(
                egui_grid::GridBuilder::new()
                    .new_row(Size::remainder())
                    .cells(Size::exact(50.), 2),
            )
            .cell(Size::relative(0.5))
            .with_layout(Layout::left_to_right(Align::Center))
            .nest(
                egui_grid::GridBuilder::new()
                    .new_row(Size::remainder())
                    .cells(Size::exact(50.), 2),
            );

        main_grid.show(ui, |mut grid| {
            grid.cell(|ui| {
                if btn_sized!(ui, 0., 10., "Load").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("robot path", &["json"])
                        .pick_file();
                    if let Some(path) = file {
                        let res = self.encoder.load(&path);
                        if let Err(err) = res {
                            self.err = Some(err.into());
                        }
                    }
                }
            });

            grid.cell(|ui| {
                if btn_sized!(ui, 0., 10., "Save").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("robot path", &["json"])
                        .save_file();
                    if let Some(path) = file {
                        let res = self.encoder.save(&path);
                        if let Err(err) = res {
                            self.err = Some(err.into());
                        }
                    }
                }
            });

            grid.cell(|ui| if btn_sized!(ui, 0., 10., "Run").clicked() {});

            grid.cell(|ui| {
                let ports = Encoder::list_ports().unwrap();
                ComboBox::from_id_source("comport-selector")
                    .selected_text(self.port.clone().unwrap_or_else(|| "None".to_string()))
                    .show_ui(ui, |ui| {
                        for port in ports {
                            if ui
                                .selectable_value(&mut self.port, Some(port.clone()), port.clone())
                                .changed()
                            {
                                let res = self.encoder.connect(port, 9600);
                                match res {
                                    Ok(_) => println!("Connected successfully"),
                                    Err(e) => println!("{e}"),
                                }
                            }
                        }
                    });
            })
        });
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        egui_grid::GridBuilder::new()
            .new_row(Size::exact(70.))
            .cells(Size::remainder(), 4)
            .new_row(Size::exact(70.))
            .cells(Size::remainder(), 4)
            .new_row(Size::exact(70.))
            .cells(Size::remainder(), 4)
            .show(ui, |mut grid| {
                grid.cell(|ui| control!(Command::GripperUp, ui, self));
                grid.cell(|ui| control!(Command::ShoulderUp, ui, self));
                grid.cell(|ui| control!(Command::ElbowUp, ui, self));
                grid.cell(|ui| {
                    if btn_sized!(ui, 5., 5., "X").clicked() {
                        self.encoder.clear_steps();
                    }
                });

                grid.cell(|ui| control!(Command::GripperDown, ui, self));
                grid.cell(|ui| control!(Command::ShoulderDown, ui, self));
                grid.cell(|ui| control!(Command::ElbowDown, ui, self));
                grid.cell(|ui| control!(Command::IPos, ui, self));

                grid.cell(|ui| control!(Command::Left, ui, self));
                grid.cell(|ui| control!(Command::GripperClose, ui, self));
                grid.cell(|ui| control!(Command::GripperOpen, ui, self));
                grid.cell(|ui| control!(Command::Right, ui, self));
            });
    }

    fn legend(&self, ui: &mut egui::Ui) {
        ui.monospace(format!("{} - {}", Command::Left, Command::Left.legend()));
        ui.monospace(format!("{} - {}", Command::Right, Command::Right.legend()));
        ui.monospace(format!("{} - {}", Command::IPos, Command::IPos.legend()));
        ui.monospace(format!(
            "{} - {}",
            Command::ShoulderDown,
            Command::ShoulderDown.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::ShoulderUp,
            Command::ShoulderUp.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::ElbowDown,
            Command::ElbowDown.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::ElbowUp,
            Command::ElbowUp.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::GripperDown,
            Command::GripperDown.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::GripperUp,
            Command::GripperUp.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::GripperClose,
            Command::GripperClose.legend()
        ));
        ui.monospace(format!(
            "{} - {}",
            Command::GripperOpen,
            Command::GripperOpen.legend()
        ));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Some(err) = &self.err {
            // self.toasts.error(err.to_string());
            dbg!(err);
            self.err = None;
        }

        egui::TopBottomPanel::top("top-menu")
            .exact_height(40.)
            .resizable(false)
            .show(ctx, |ui| {
                // ui.spacing_mut().item_spacing.x = 5.;

                ui.with_layout(Layout::top_down(Align::Center), |ui| self.top_menu(ui));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_grid::GridBuilder::new()
                .new_row(Size::relative(0.5))
                .cells(Size::remainder(), 2)
                .new_row(Size::relative(0.5))
                .cells(Size::remainder(), 2)
                .show(ui, |mut grid| {
                    grid.cell(|ui| self.controls(ui));

                    grid.cell(|ui| {
                        ui.monospace(self.encoder.localize());
                    });

                    grid.cell(|ui| self.legend(ui));

                    grid.empty();
                });
        });

        // self.toasts.show(ctx);
    }
}
