use chrono::{Datelike};
use eframe::egui::{self};
use itertools::Itertools;

use crate::BufferId;


fn month_to_name(m: u32) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Now",
        12 => "Dec",
        _ => "???",
    }
}

pub fn show_note_tree(buffers: &Vec::<BufferId>, ui: &mut egui::Ui) -> Option<BufferId> {
    let mut selected = None;
    let by_year = buffers.iter().group_by(|a|  a.date.year());
    for (year, group) in &by_year {
        ui.collapsing(year.to_string(), |ui| {
            let by_month = group.group_by(|a| a.date.month());
            for (month, group) in &by_month {
                let month_name = month_to_name(month);
                ui.collapsing(month_name, |ui| {
                    for d in group.sorted_by(|a, b| Ord::cmp(&a.date, &b.date)) {
                        let name = d.date.day().to_string();
                        if ui.button(name).clicked() {
                            selected = Some(d.clone());
                        }
                    }
                });
            }            
        });
    }
    selected
}