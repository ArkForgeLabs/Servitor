pub mod runtime;
pub mod tests;

fn main() -> anyhow::Result<()> {
    simple_logging::log_to_stderr(log::LevelFilter::Info);

    Ok(())
}
