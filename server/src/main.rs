use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder, facades::QuickJsRuntimeFacade, jsutils::Script,
    values::JsValueConvertable,
};

fn main() -> anyhow::Result<()> {
    simple_logging::log_to_stderr(log::LevelFilter::Info);
    let rt: QuickJsRuntimeFacade = QuickJsRuntimeBuilder::new().build();

    let test_script = include_str!("../example/test.js");
    rt.eval_sync(None, Script::new("test.js", test_script))?;

    rt.invoke_function_sync(
        None,
        &["scripts", "bot1"],
        "sayHello",
        vec![2i32.to_js_value_facade(), 3i32.to_js_value_facade()],
    )?;

    Ok(())
}
