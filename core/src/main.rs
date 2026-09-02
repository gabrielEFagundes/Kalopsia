use ratatui::{DefaultTerminal, Frame};
use core::runtime::Runtime;

fn main() -> std::io::Result<()>{
    let _runtime = Runtime::new();
    
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()>{
    loop{
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press(){
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame){
    frame.render_widget("Kalopsia's first widget", frame.area());
}