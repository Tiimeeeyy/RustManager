mod process_info;
mod tui;
mod system_info;

use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{Stdout, Write};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rayon::prelude::*;
use system_info::*;

const SORT_INTERVAL: Duration = Duration::from_secs(2);
const REFRESH_INTERVAL: Duration = Duration::from_secs(1);

pub struct App {
    processes: Vec<process_info::ProcessInfo>,
    cpu_usage: f32,
    used_memory: u64,
    cpu_history: Vec<f32>,
    mem_history: Vec<u64>,
    process_map: HashMap<sysinfo::Pid, usize>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut app = create_app();
    let mut last_update = Instant::now();
    let mut last_sort = Instant::now();
    update_tui(&mut terminal, &mut app, last_update, last_sort)?;


    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn update_tui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut app: &mut App, mut last_update: Instant, mut last_sort: Instant) -> Result<(), Box<dyn Error>> {
    loop {
        let now = Instant::now();
        if now.duration_since(last_sort) >= SORT_INTERVAL {
            app.processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
            app.process_map = app.processes
                .iter()
                .enumerate()
                .map(|(i, process)| (process.pid, i))
                .collect();
            last_sort = now;
        }
        if now.duration_since(last_update) >= SORT_INTERVAL {
            let (processes, cpu_usage, used_mem) = fetch_system_info();
            app.processes = processes;
            app.cpu_usage = cpu_usage;
            app.used_memory = used_mem;
            app.cpu_history.push(cpu_usage);
            app.mem_history.push(used_mem);

            if app.cpu_history.len() > 100 {
                app.cpu_history.remove(0);
            }
            if app.mem_history.len() > 100 {
                app.mem_history.remove(0);
            }
        }
        terminal.draw(|f| tui::ui(f, &mut app))?;
        io::stdout().flush().unwrap();

        if let Ok(true) = event::poll(Duration::from_millis(50)) {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {} // Handle other key events if needed
                }
            }
        }
    }
    Ok(())
}

fn create_app() -> App {
    let mut app = App {
        processes: Vec::new(),
        cpu_usage: 0.0,
        used_memory: 0,
        cpu_history: Vec::new(),
        mem_history: Vec::new(),
        process_map: HashMap::new(),
    };
    app
}