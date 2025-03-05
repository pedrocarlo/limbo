#![allow(clippy::arc_with_non_send_sync)]
mod app;
mod helper;
mod import;
mod input;
mod opcodes_dictionary;
mod readline;
mod readline_utils;

use reedline::{DefaultPrompt, DefaultPromptSegment, Prompt, Reedline};
use rustyline::{error::ReadlineError, Config, Editor};
use std::sync::atomic::Ordering;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn rustyline_config() -> Config {
    Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build()
}

fn main() -> anyhow::Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let sig = line_editor.read_line(&prompt);

    let mut rl = Editor::with_config(rustyline_config())?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_thread_ids(true),
        )
        .with(EnvFilter::from_default_env())
        .init();
    let mut app = app::Limbo::new(&mut rl)?;
    let home = dirs::home_dir().expect("Could not determine home directory");
    let history_file = home.join(".limbo_history");
    if history_file.exists() {
        app.rl.load_history(history_file.as_path())?;
    }
    loop {
        let readline = app.rl.readline(&app.prompt);
        match readline {
            Ok(line) => match app.handle_input_line(line.trim()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            Err(ReadlineError::Interrupted) => {
                // At prompt, increment interrupt count
                if app.interrupt_count.fetch_add(1, Ordering::SeqCst) >= 1 {
                    eprintln!("Interrupted. Exiting...");
                    let _ = app.close_conn();
                    break;
                }
                println!("Use .quit to exit or press Ctrl-C again to force quit.");
                app.reset_input();
                continue;
            }
            Err(ReadlineError::Eof) => {
                app.handle_remaining_input();
                let _ = app.close_conn();
                break;
            }
            Err(err) => {
                let _ = app.close_conn();
                anyhow::bail!(err)
            }
        }
    }
    rl.save_history(history_file.as_path())?;
    Ok(())
}
