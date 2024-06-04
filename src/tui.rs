use crate::{traits::kubernetes::Kubernetes, PodUI};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use ratatui::{backend::CrosstermBackend, style::Style};
use ratatui::{layout::Alignment, Terminal};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};
use ratatui::{
    style::Modifier,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{collections::HashMap, thread};
use std::{
    collections::HashSet,
    io::{self, stdout},
};
use std::{rc::Rc, time::Duration};

use crate::kubernetes_kubectl_implementation::KubernetesImpl;

fn init() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore(_terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn create_main_layout(size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Max(3),
            Constraint::Min(1),
        ])
        .split(size)
}

fn create_namespace_layout(main_layout: &[Rect], environment: &str) -> Rc<[Rect]> {
    let space: u16 = environment.chars().count().try_into().unwrap();
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(space + 2), Constraint::Min(1)])
        .split(main_layout[1])
}

fn create_pods_layout(main_layout: &[Rect]) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(main_layout[2])
}

fn render_header(f: &mut ratatui::Frame, main_layout: &[Rect]) {
    let header = Block::default()
        .title("* KFORWARD *")
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP);

    f.render_widget(header, main_layout[0]);
}

fn render_namespace(f: &mut ratatui::Frame, namespace_layout: &[Rect], environment: &str) {
    let current_namespace_paragraph = Paragraph::new(environment)
        .block(Block::default().title("Namespace").borders(Borders::ALL));

    f.render_widget(current_namespace_paragraph, namespace_layout[0]);
}

fn render_pod_list(
    f: &mut ratatui::Frame,
    pods_layout: &[Rect],
    filtered_pods: &mut Vec<&mut PodUI>,
    // &mut [&mut PodUI],
    selected_index: usize,
) {
    let items: Vec<ListItem> = filtered_pods
        .iter_mut()
        .enumerate()
        .map(|(i, pod)| {
            let label = format!(
                "{} - {} - {}:{} - {}",
                pod.get_context(),
                pod.get_namespace(),
                pod.get_service(),
                pod.get_port(),
                if pod.is_running() {
                    "Running"
                } else {
                    "Not Running"
                }
            );
            let style = match (i == selected_index, pod.is_running()) {
                (true, _) => Style::default().fg(Color::Yellow),
                (_, true) => Style::default().fg(Color::Green),
                (_, false) => Style::default().fg(Color::LightRed),
            };

            ListItem::new(label).style(style)
        })
        .collect();

    let pod_list = List::new(items)
        .block(Block::default().title("Pods").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));

    f.render_widget(pod_list, pods_layout[0]);
}

pub async fn run(mut pods: Vec<PodUI>) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = init()?;

    let contexts: Vec<String> = pods
        .iter()
        .map(|pod| pod.get_context())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Initialize selected_indices for each context
    let mut selected_indices: HashMap<String, usize> = contexts
        .iter()
        .map(|context| (context.clone(), 0))
        .collect();

    let mut environment_position = 0;
    let mut environment: String = contexts.get(0).map(|t| t.clone()).unwrap_or_default();

    loop {
        // Filter pods based on the current environment
        let mut filtered_pods: Vec<&mut PodUI> = pods
            .iter_mut()
            .filter(|pod| pod.get_context() == environment)
            .collect();

        // Get the selected index for the current environment
        let selected_index = selected_indices.entry(environment.clone()).or_insert(0);

        terminal.draw(|f| {
            let size = f.size();
            let main_layout = create_main_layout(size);
            let namespace_layout = create_namespace_layout(&main_layout, &environment);
            let pods_layout = create_pods_layout(&main_layout);

            render_header(f, &main_layout);
            render_namespace(f, &namespace_layout, &environment);
            render_pod_list(f, &pods_layout, &mut filtered_pods, *selected_index);
        })?;

        // Handle keyboard events
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    let _ = kill_running_pods(&mut pods);
                    break;
                }
                KeyCode::Up => {
                    if !filtered_pods.is_empty() {
                        *selected_index =
                            (*selected_index + filtered_pods.len() - 1) % filtered_pods.len();
                    }
                }
                KeyCode::Down => {
                    if !filtered_pods.is_empty() {
                        *selected_index = (*selected_index + 1) % filtered_pods.len();
                    }
                }
                KeyCode::Char('k') => {
                    if !filtered_pods.is_empty() {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        } else {
                            *selected_index = filtered_pods.len() - 1;
                        }
                    }
                }
                KeyCode::Char('j') => {
                    if !filtered_pods.is_empty() {
                        *selected_index = (*selected_index + 1) % filtered_pods.len();
                    }
                }
                KeyCode::Char('e') => {
                    environment_position = (environment_position + 1) % contexts.len();
                    environment = contexts.get(environment_position).unwrap().clone();
                }
                KeyCode::Enter => {
                    if !filtered_pods.is_empty() {
                        let selected_item = &mut filtered_pods[*selected_index];
                        process_selected_pod(selected_item)?;
                    }
                }
                _ => {}
            }
        }

        // Add a short delay to prevent high CPU usage
        thread::sleep(Duration::from_millis(50));
    }
    restore(&mut terminal)?;

    Ok(())
}

fn process_selected_pod(pod: &mut PodUI) -> Result<(), Box<dyn std::error::Error>> {
    // let rt = Runtime::new().unwrap();
    let pod_clone = pod.pod.clone();
    if pod.is_running() {
        let _ = pod.stop();
        pod.process = None;
    } else {
        pod.process = KubernetesImpl::forward_connection(&pod_clone).ok();
    }
    Ok(())
}

fn kill_running_pods(pods: &mut Vec<PodUI>) -> Result<(), Box<dyn std::error::Error>> {
    for pod in pods.iter_mut() {
        if pod.is_running() {
            pod.stop()?;
        }
    }
    Ok(())
}
