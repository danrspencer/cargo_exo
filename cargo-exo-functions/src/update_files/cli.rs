use crate::update_files::{LineAction, LineUpdate};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use difference::{Changeset, Difference};

pub trait Cli {
    fn display_error(error: &str);

    fn confirm_update(file: &str, line_update: &LineUpdate, lines: &[String]) -> bool;

    fn confirm_update_2(filename: &str, original_contents: &str, updated_contents: &str) -> bool;
}

pub struct UserCli;

impl Cli for UserCli {
    fn display_error(error: &str) {
        println!();

        let (mut error, message) = error.split_at(error.find(':').unwrap_or(0));
        if error.is_empty() {
            error = "error";
        }
        println!(
            "{}{} {}",
            error.bright_red().bold(),
            ":".bold(),
            message.bold()
        );
    }

    fn confirm_update(file: &str, line_update: &LineUpdate, lines: &[String]) -> bool {
        let index = line_update.line_no as usize - 1;
        let indent = " ".repeat(index.to_string().len());

        println!(
            "{}{} {}:{}",
            indent,
            "-->".bright_blue().bold(),
            file,
            line_update.line_no
        );

        match line_update.action {
            LineAction::Insert => {
                if let Some(ref content) = line_update.content {
                    println!(
                        "{} {}",
                        format!("{} |", line_update.line_no).bright_blue().bold(),
                        format!("+{}", content).green()
                    );
                }
            }
            LineAction::Replace => {
                if let Some(ref content) = line_update.content {
                    println!(
                        "{} {}",
                        format!("{} |", line_update.line_no).bright_blue().bold(),
                        format!("-{}", lines[index]).red()
                    );
                    println!(
                        "{} {}",
                        format!("{} |", line_update.line_no).bright_blue().bold(),
                        format!("+{}", content).green()
                    );
                }
            }
            LineAction::Delete => {
                println!(
                    "{} {}",
                    format!("{} |", line_update.line_no).bright_blue().bold(),
                    format!("-{}", lines[index]).red()
                );
            }
        }

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to apply these changes?")
            .default(true)
            .interact()
            .unwrap()
    }

    fn confirm_update_2(filename: &str, original_contents: &str, updated_contents: &str) -> bool {
        let changeset = Changeset::new(original_contents, updated_contents, "\n");

        let mut original_line_no = 1;
        let mut updated_line_no = 1;

        let mut last_change_line_no = 0;

        for diff in &changeset.diffs {
            let change = match diff {
                Difference::Same(ref x) => {
                    let lines = x.matches('\n').count() + 1;
                    original_line_no += lines;
                    updated_line_no += lines;
                    None
                }
                Difference::Add(ref x) => {
                    if !x.is_empty() {
                        updated_line_no += 1;
                        Some((updated_line_no - 1, format!("+{}", x).green()))
                    } else {
                        None
                    }
                }
                Difference::Rem(ref x) => {
                    original_line_no += 1;
                    Some((original_line_no - 1, format!("-{}", x).red()))
                }
            };

            if let Some((line, change)) = change {
                let indent_size = vec![original_line_no, updated_line_no, line]
                    .iter()
                    .max()
                    .unwrap()
                    .to_string()
                    .len();
                let indent = " ".repeat(indent_size);

                if original_line_no - last_change_line_no > 1 {
                    println!(
                        "{}{} {}:{}",
                        indent,
                        "-->".bright_blue().bold(),
                        filename,
                        original_line_no
                    );
                }
                println!("{} {}", format!("{} |", line).bright_blue().bold(), change);
                last_change_line_no = original_line_no;
            }
        }

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to apply these changes?")
            .default(true)
            .interact()
            .unwrap()
    }
}
