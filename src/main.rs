use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use structopt::{clap::AppSettings, StructOpt};

pub fn run(cmd: &[&str]) -> Result<()> {
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Could not run command {:?}", cmd))?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Command {:?} failed: {:?}", cmd, status);
    }
}

fn run_with_output(cmd: &[&str]) -> Result<String> {
    let output = Command::new(cmd[0])
        .args(&cmd[1..])
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("Could not run command {:?}", cmd))?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        anyhow::bail!(
            "Command {:?} failed with exit code: {:?}",
            cmd,
            output.status.code()
        );
    }
}

#[derive(Debug)]
struct Screen {
    name: String,
    is_primary: bool,
    is_connected: bool,
    first_resolution: Option<(usize, usize)>,
}

fn parse_xrandr() -> Result<Vec<Screen>> {
    let xrandr_output = run_with_output(&["xrandr"])?;

    let mut out: Vec<Screen> = Vec::new();
    let mut cur: Option<Screen> = None;

    for line in xrandr_output.lines() {
        if line.contains("connected ") {
            let is_primary = line.contains(" primary ");
            let is_connected = line.contains(" connected ");
            let name = line.split(" ").next().unwrap().to_string();
            if let Some(cur) = cur.take() {
                out.push(cur);
            }
            cur = Some(Screen {
                name,
                is_primary,
                is_connected,
                first_resolution: None,
            });
        } else if line.starts_with(" ") {
            if let Some(mut cur) = cur.take() {
                let resolution = line
                    .trim()
                    .split(" ")
                    .next()
                    .unwrap()
                    .split("x")
                    .collect::<Vec<_>>();
                cur.first_resolution = Some((resolution[0].parse()?, resolution[1].parse()?));
                out.push(cur);
            }
        }
    }

    Ok(out)
}

#[derive(Debug, StructOpt)]
#[structopt(
    global_setting = AppSettings::InferSubcommands,
    global_setting = AppSettings::ColoredHelp,
    global_setting = AppSettings::VersionlessSubcommands,
    name = "xrandr-helper",
    about = "A wrapper around xrandr for sane defaults",
)]
struct Args {
    #[structopt(long)]
    dpi: Option<u32>,

    #[structopt(subcommand)]
    pub command: ArgCommand,
}
#[derive(Debug, StructOpt)]
enum ArgCommand {
    Normal,
    Single,
}

fn set_as_single(all_displays: &[Screen], chosen: &Screen, dpi: Option<u32>) -> Result<()> {
    let resolution = chosen
        .first_resolution
        .ok_or_else(|| anyhow::anyhow!("Chosen display does not have a resolution"))?;

    for display in all_displays {
        if display.name != chosen.name {
            run(&["xrandr", "--output", &display.name, "--off"])?;
        }
    }

    let resolution = format!("{}x{}", resolution.0, resolution.1);
    let mut args = vec![
        "xrandr",
        "--output",
        &chosen.name,
        "--auto",
        "--mode",
        &resolution,
        "--scale-from",
        &resolution,
    ];
    let dpi_str;
    if let Some(dpi) = dpi {
        args.push("--dpi");
        dpi_str = format!("{}", dpi);
        args.push(&dpi_str);
    }

    run(&args)?;

    Ok(())
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    match args.command {
        ArgCommand::Normal => {
            let displays = parse_xrandr()?;
            if let Some(chosen) = displays
                .iter()
                .filter(|d| d.first_resolution.is_some())
                .max_by_key(|d| (d.is_primary, d.first_resolution))
            {
                set_as_single(&displays, chosen, args.dpi)?;
            } else {
                anyhow::bail!("Cannot find a display");
            }
        }
        ArgCommand::Single => {
            let displays = parse_xrandr()?;
            if let Some(chosen) = displays
                .iter()
                .filter(|d| d.first_resolution.is_some())
                .max_by_key(|d| (d.first_resolution, d.is_primary))
            {
                set_as_single(&displays, chosen, args.dpi)?;
            } else {
                anyhow::bail!("Cannot find a display");
            }
        }
    }
    Ok(())
}
