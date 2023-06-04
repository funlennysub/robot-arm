use crate::error::Error;
use eframe::{
    egui::{self, Context, Layout},
    emath::Align,
};
use egui_extras::Size;
use encoder_lib::{command::Command, encoder::Encoder};

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
            $slf.encoder.commands.push(cmd);
            let _res = cmd.run(&mut $slf.encoder.port);
        }
    }};
}

pub(crate) struct App {
    // pub(crate) toasts: Toasts,
    pub(crate) encoder: Encoder,
    pub(crate) err: Option<Error>,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .iter_mut()
            .for_each(|font| font.1.tweak.scale = 1.3);

        cc.egui_ctx.set_fonts(fonts);

        Self {
            // toasts: Toasts::default(),
            encoder: Encoder::default(),
            err: None,
        }
    }

    fn top_menu(&mut self, ui: &mut egui::Ui) {
        egui_grid::GridBuilder::new()
            .new_row(Size::relative(1.))
            .cells(Size::exact(50.), 2)
            .with_layout(Layout::left_to_right(Align::Center))
            .show(ui, |mut grid| {
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
            });
    }
    // https://canary.discord.com/channels/@me/984127963727536148/1070080098537066516
    fn controls(&mut self, ui: &mut egui::Ui) {
        egui_grid::GridBuilder::new()
            .new_row(Size::exact(40.))
            .cells(Size::remainder(), 4)
            .new_row(Size::exact(40.))
            .cells(Size::remainder(), 4)
            .new_row(Size::exact(40.))
            .cells(Size::remainder(), 4)
            .show(ui, |mut grid| {
                grid.cell(|ui| control!(Command::GripperUp(1), ui, self));
                grid.cell(|ui| control!(Command::ShoulderUp(1), ui, self));
                grid.cell(|ui| control!(Command::ElbowUp(1), ui, self));
                grid.cell(|ui| {
                    if btn_sized!(ui, 5., 5., "X").clicked() {
                        self.encoder.clear_commands();
                    }
                });

                grid.cell(|ui| control!(Command::GripperDown(1), ui, self));
                grid.cell(|ui| control!(Command::ShoulderDowm(1), ui, self));
                grid.cell(|ui| control!(Command::ElbowDown(1), ui, self));
                grid.cell(|ui| control!(Command::IPos(1), ui, self));

                grid.cell(|ui| control!(Command::Left(1), ui, self));
                grid.cell(|ui| control!(Command::GripperClose(1), ui, self));
                grid.cell(|ui| control!(Command::GripperOpen(1), ui, self));
                grid.cell(|ui| control!(Command::Right(1), ui, self));
            });
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
                ui.spacing_mut().item_spacing.x = 5.;

                ui.with_layout(Layout::top_down(Align::Center), |ui| self.top_menu(ui));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_grid::GridBuilder::new()
                .new_row(Size::relative(0.5))
                .cells(Size::remainder(), 2)
                .show(ui, |mut grid| {
                    grid.cell(|ui| self.controls(ui));

                    grid.cell(|ui| {
                        ui.monospace(
                            self.encoder
                                .commands
                                .iter()
                                .map(|c| format!("{c:?}"))
                                .collect::<Vec<_>>()
                                .join(""),
                        );
                    });
                });
        });

        // self.toasts.show(ctx);
    }
}
