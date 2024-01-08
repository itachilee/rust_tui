use std::{
    collections::HashMap,
    io::{stderr, Result},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use app::{App, CurrentScreen, CurrentlyEditing};
use ui::ui;

use serde_json::Value;
fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr))?;
    terminal.clear()?;

    let mut app = App::new();

    let file = std::fs::File::open("./db.json").expect("file not found");

    let rs = std::io::read_to_string(file).expect("not a string");

    // let mut hashmap: HashMap<String, String> = HashMap::new();

    // 将 JSON 字符串数组解析为 serde_json::Value
    let hashmap: HashMap<String, String> = serde_json::from_str(&rs)?;

    // 遍历解析后的 JSON 数组
    // for json_obj in parsed {
    //     // 对每个 JSON 对象执行操作
    //     if let Value::Object(map) = json_obj {
    //         for (key, value) in map {
    //             if let Value::String(v) = value {
    //                 // 将 key-value 对插入 HashMap
    //                 hashmap.insert(key, v);
    //             }
    //         }
    //     }
    // }
    app.pairs = hashmap;
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        if event::poll(std::time::Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                // dbg!(key.code);
                if key.kind == KeyEventKind::Press {
                    // windows 按键按下与抬起都会触发一次时间，但linux/mac 不会，所以只监听Press 事件就可以了

                    // 绘制不同的ui
                    match app.current_screen {
                        CurrentScreen::Main => match key.code {
                            KeyCode::Char('e') => {
                                app.current_screen = CurrentScreen::Editing;
                                app.currently_editing = Some(CurrentlyEditing::Key);
                            }
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.current_screen = CurrentScreen::Exiting;
                            }
                            _ => {}
                        },

                        CurrentScreen::Exiting => match key.code {
                            KeyCode::Char('y') => {
                                return Ok(true);
                            }
                            KeyCode::Char('n') | KeyCode::Char('q') => {
                                return Ok(false);
                            }
                            _ => {}
                        },
                        CurrentScreen::Editing => match key.code {
                            KeyCode::Enter => {
                                //  按下enter 键 如果状态为编辑key,则跳转到编辑value
                                //  若状态为value，则退出编辑状态回到main
                                if let Some(editing) = &app.currently_editing {
                                    match editing {
                                        CurrentlyEditing::Key => {
                                            app.currently_editing = Some(CurrentlyEditing::Value);
                                        }
                                        CurrentlyEditing::Value => {
                                            app.save_key_value();
                                            app.current_screen = CurrentScreen::Main;
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                if let Some(editing) = &app.currently_editing {
                                    match editing {
                                        CurrentlyEditing::Key => {
                                            app.key_input.pop();
                                        }
                                        CurrentlyEditing::Value => {
                                            app.value_input.pop();
                                        }
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                app.current_screen = CurrentScreen::Main;
                                app.currently_editing = None;
                            }
                            KeyCode::Tab => {
                                app.toggle_editing();
                            }
                            KeyCode::Char(value) => {
                                if let Some(editing) = &app.currently_editing {
                                    match editing {
                                        CurrentlyEditing::Key => {
                                            app.key_input.push(value);
                                        }
                                        CurrentlyEditing::Value => {
                                            app.value_input.push(value);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }
    Ok(true)
}
