#[derive(serde::Serialize, serde::Deserialize)]
struct TestStructure {
    a: i32,
    b: i32,
}

#[test]
fn main() -> anyhow::Result<()> {
    simple_logging::log_to_stderr(log::LevelFilter::Info);
    let mut runtime = crate::runtime::JSRuntime::new();

    let test_script = include_str!("./js_runtime.js");

    runtime.insert_script(
        "test.js",
        test_script,
        vec![serde_json::to_value(TestStructure { a: 1, b: 2 })?],
    );
    runtime.run_scripts()?;

    Ok(())
}
