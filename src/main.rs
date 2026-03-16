mod cli;
mod errors;
mod hyprctl;
mod shader;
mod theme;

use anyhow::{Context, Result};
use clap::Parser;

use cli::Cli;
use errors::FocusError;

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    if cli.list {
        print_themes();
        return Ok(());
    }

    if cli.off {
        hyprctl::check_environment()?;
        hyprctl::clear_shader().context("failed to clear shader")?;
        shader::cleanup_shaders().context("failed to clean up shader files")?;
        return Ok(());
    }

    if let Some(ref name) = cli.theme {
        let theme =
            theme::find_theme(name).ok_or_else(|| FocusError::UnknownTheme(name.clone()))?;

        hyprctl::check_environment()?;

        // Clear existing shader so Hyprland detects the change
        if let Err(e) = hyprctl::clear_shader() {
            log::warn!("Failed to clear previous shader: {e}");
        }
        shader::cleanup_shaders().context("failed to clean up old shader files")?;

        let shader_path = shader::write_shader(theme, cli.opacity, cli.brightness, cli.saturation)
            .context("failed to write shader")?;
        hyprctl::set_shader(&shader_path).context("failed to apply shader")?;

        log::info!("Applied '{}'", theme.name);
        return Ok(());
    }

    Ok(())
}

fn print_themes() {
    println!("Available themes:\n");
    for theme in theme::builtin_themes() {
        let (lo, hi) = theme.wavelength_range;
        let wavelength = format!("{lo}-{hi} nm");
        println!(
            "  {:<12} {:<14} {}",
            theme.name, wavelength, theme.description
        );
    }
    println!();
    println!("Usage: focus --theme <NAME> [--opacity] [--brightness] [--saturation]");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn run_list() {
        let cli = Cli::try_parse_from(["focus", "--list"]).unwrap();
        assert!(run(cli).is_ok());
    }

    #[test]
    fn run_unknown_theme() {
        let cli = Cli::try_parse_from(["focus", "--theme", "nonexistent"]).unwrap();
        let err = run(cli).unwrap_err();
        assert!(err.downcast_ref::<FocusError>().is_some());
    }

    #[test]
    #[serial]
    fn run_off_without_hyprland() {
        unsafe { std::env::remove_var("HYPRLAND_INSTANCE_SIGNATURE") };
        let cli = Cli::try_parse_from(["focus", "--off"]).unwrap();
        let err = run(cli).unwrap_err();
        assert!(err.downcast_ref::<FocusError>().is_some());
    }

    #[test]
    #[serial]
    fn run_theme_without_hyprland() {
        unsafe { std::env::remove_var("HYPRLAND_INSTANCE_SIGNATURE") };
        let cli = Cli::try_parse_from(["focus", "--theme", "military"]).unwrap();
        let err = run(cli).unwrap_err();
        assert!(err.downcast_ref::<FocusError>().is_some());
    }
}
