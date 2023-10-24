extern crate sysinfo;
extern crate battery;
extern crate tui;
extern crate crossterm;

use std::io;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, ProcessorExt};
use battery::Manager;
use battery::units::ratio::percent;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use crossterm::event::{self, KeyCode};

enum Section {
    Welcome,
    Metrics,
    Instructions,
    DetailedMetrics,
    CPUDetails,
    MemoryDetails,
    BatteryDetails,
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut current_section = Section::Welcome;
    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(2);
    let mut needs_redraw = true;

    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                event::Event::Key(event) => {
                    needs_redraw = true;
                    match event.code {
                        KeyCode::Char('q') => {
                            crossterm::terminal::disable_raw_mode().unwrap();
                            terminal.clear().unwrap();
                            return;
                        }
                        KeyCode::Up => {
                            current_section = match current_section {
                                Section::Instructions => Section::Metrics,
                                Section::Metrics => Section::Welcome,
                                Section::Welcome => Section::Welcome,
                                Section::DetailedMetrics => Section::BatteryDetails,
                                Section::CPUDetails => Section::DetailedMetrics,
                                Section::MemoryDetails => Section::CPUDetails,
                                Section::BatteryDetails => Section::MemoryDetails,
                            };
                        }
                        KeyCode::Down => {
                            current_section = match current_section {
                                Section::Welcome => Section::Metrics,
                                Section::Metrics => Section::Instructions,
                                Section::Instructions => Section::Instructions,
                                Section::DetailedMetrics => Section::CPUDetails,
                                Section::CPUDetails => Section::MemoryDetails,
                                Section::MemoryDetails => Section::BatteryDetails,
                                Section::BatteryDetails => Section::DetailedMetrics,
                            };
                        }
                        KeyCode::Enter => {
                            match current_section {
                                Section::Metrics => current_section = Section::DetailedMetrics,
                                Section::DetailedMetrics => current_section = Section::CPUDetails,
                                Section::CPUDetails => current_section = Section::MemoryDetails,
                                Section::MemoryDetails => current_section = Section::BatteryDetails,
                                Section::BatteryDetails => current_section = Section::Welcome,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if needs_redraw || last_update.elapsed() > update_interval {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(0),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let block = match current_section {
                    Section::Welcome => Block::default().borders(Borders::ALL).title("Welcome").style(Style::default().fg(Color::Yellow)),
                    _ => Block::default().borders(Borders::ALL).title("Welcome"),
                };
                let welcome = Paragraph::new("NovaMetrics").block(block).wrap(Wrap { trim: true });
                f.render_widget(welcome, chunks[0]);

                let metrics = format!(
                    "CPU: {:.2}%\nMemory: {} KB\nBattery: {}%",
                    get_cpu_usage(),
                    get_used_memory(),
                    get_battery_percentage()
                );
                let block = match current_section {
                    Section::Metrics => Block::default().borders(Borders::ALL).title("Metrics").style(Style::default().fg(Color::Yellow)),
                    _ => Block::default().borders(Borders::ALL).title("Metrics"),
                };
                let metrics_widget = Paragraph::new(metrics).block(block).wrap(Wrap { trim: true });
                f.render_widget(metrics_widget, chunks[1]);

                let instructions = "Use arrow keys to navigate. Press 'q' to quit.";
                let block = match current_section {
                    Section::Instructions => Block::default().borders(Borders::ALL).title("Instructions").style(Style::default().fg(Color::Yellow)),
                    _ => Block::default().borders(Borders::ALL).title("Instructions"),
                };
                let instructions_widget = Paragraph::new(instructions).block(block).wrap(Wrap { trim: true });
                f.render_widget(instructions_widget, chunks[2]);
            }).unwrap();
            last_update = Instant::now();
            needs_redraw = false;
        }
    }
}

fn get_cpu_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();
    let total_cpu_usage: f32 = system.processors().iter().map(|p| p.cpu_usage()).sum();
    total_cpu_usage / system.processors().len() as f32
}

fn get_used_memory() -> u64 {
    let mut system = System::new_all();
    system.refresh_all();
    system.used_memory()
}

fn get_battery_percentage() -> f32 {
    let manager = Manager::new().unwrap();
    if let Some(battery) = manager.batteries().unwrap().next() {
        let battery = battery.unwrap();
        battery.state_of_charge().get::<percent>()
    } else {
        0.0
    }
}
