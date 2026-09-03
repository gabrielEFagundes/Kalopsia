use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::canvas::{Canvas, Line, Points};
use ratatui::Frame;
use core::kalopsia_tui::block::KBlock;
use core::runtime::Runtime;
use core::tests;

fn main() -> std::io::Result<()>{
    let mut _runtime = Runtime::new();
    
    tests::main_test(&mut _runtime);

    let nodes_pos = lib::pg_engine::formulas::Formulas::Circular.convert((0.0, 100.0), (0.0, 100.0), _runtime.graph.nodes().len());
    
    ratatui::run(|terminal| {
        loop{
            terminal.draw(|frame| { render(frame, &nodes_pos) })?;

            if crossterm::event::read()?.is_key_press(){
                break Ok(())
            }
        }
    })
}

// -- everything below `main` is testing --

fn render(frame: &mut Frame, nodes_positionment: &Vec<(f64, f64)>){
    let [node_area, editor_area] = Layout::horizontal([
        Constraint::Percentage(70),
        Constraint::Percentage(70)
    ]).areas(frame.area());

    KBlock::new(node_area, Style::new().light_yellow())
            .render(frame);
    KBlock::new(editor_area, Style::new().light_magenta())
            .render(frame);

    frame.render_widget(Canvas::default()
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|c| {
            for (x, y) in nodes_positionment{
                c.draw(&Points{ coords: &[(*x, *y)], color: Color::LightYellow });
            }
            // for (a, b) in &nodes_conns{
            //     c.draw(&Line{ x1: nodes_pos[*a].0, y1: nodes_pos[*a].1, x2: nodes_pos[*b].0, y2: nodes_pos[*b].1, color: Color::White });
            // }
        }), node_area);
}