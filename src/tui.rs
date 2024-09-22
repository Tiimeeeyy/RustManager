use crate::App;
use ratatui::{prelude::*, widgets::*};
use rayon::prelude::*;
use sysinfo::System;

pub fn ui(f: &mut Frame, app: &mut App) {
    let mut system = System::new();
    system.refresh_all();
    let total_mem = system.total_memory();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    render_process_list(f, app, chunks[0]);
    let performance_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let cpu_history_u64: Vec<u64> = app.cpu_history.iter().map(|&x| (x) as u64).collect();
    let cpu_sparkline = Sparkline::default()
        .block(Block::default().title("CPU").borders(Borders::ALL))
        .data(&cpu_history_u64)
        .max(100)
        .style(Style::default().fg(Color::LightBlue));
    let mem_sparkline = Sparkline::default()
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .data(&app.mem_history)
        .max(total_mem)
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(cpu_sparkline, performance_chunks[0]);
    f.render_widget(mem_sparkline, performance_chunks[1]);
}

fn render_process_list(f: &mut Frame, app: &mut App, area: Rect) {
    let mut rows: Vec<Row> = Vec::with_capacity(app.processes.len());

    for (&pid, &row_index) in &app.process_map {
        if let Some(process) = app.processes.iter().find(|p| p.pid == pid) {
            let insertion_index = if row_index < rows.len() {
                row_index
            } else {
                rows.len()
            };
            rows.insert(
                insertion_index,
                Row::new(vec![
                    Cell::from(process.pid.to_string()),
                    Cell::from(process.name.clone()),
                    Cell::from(format!("{:.1}%", process.cpu_usage / 10.0)),
                    Cell::from(format!("{} MB", process.memory_usage / 1024 / 1024)),
                    Cell::from(format!("{:?}", process.status)),
                ]),
            );
        }
    }
    for (i, process) in app.processes.iter().enumerate() {
        if let std::collections::hash_map::Entry::Vacant(e) = app.process_map.entry(process.pid) {
            e.insert(rows.len());
            rows.push(Row::new(vec![
                Cell::from(process.pid.to_string()),
                Cell::from(process.name.clone()),
                Cell::from(format!("{:.1}%", process.cpu_usage)),
                Cell::from(format!("{} MB", process.memory_usage / 1024 / 1024)),
                Cell::from(format!("{:?}", process.status)),
            ]));
        }
    }
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("CPU"),
        Cell::from("Memory"),
        Cell::from("Status"),
    ]);
    let widths = (vec![
        Constraint::Percentage(10),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
    ]);
    let process_table = Table::new(rows.into_iter(), widths)
        .header(header)
        .block(Block::default().title("Processes").borders(Borders::ALL));

    f.render_widget(process_table, area);
}
