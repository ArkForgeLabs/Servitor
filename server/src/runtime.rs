use std::collections::HashMap;

use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder, facades::QuickJsRuntimeFacade, jsutils::Script,
    values::JsValueConvertable,
};

pub struct JSRuntime {
    pub runtime: QuickJsRuntimeFacade,
    pub scripts: HashMap<String, (String, Vec<serde_json::Value>)>,
}

impl JSRuntime {
    pub fn new() -> Self {
        Self {
            runtime: QuickJsRuntimeBuilder::new().build(),
            scripts: HashMap::new(),
        }
    }

    pub fn insert_script(&mut self, name: &str, script: &str, args: Vec<serde_json::Value>) {
        self.scripts
            .insert(name.to_string(), (script.to_string(), args));
    }

    pub fn delete_script(&mut self, name: &str) {
        self.scripts.remove(name);
    }

    pub fn run_scripts(&self) -> anyhow::Result<()> {
        for i in self.scripts.iter() {
            self.runtime
                .eval_sync(None, Script::new(i.0.as_str(), i.1 .0.as_str()))?;

            self.runtime.invoke_function_sync(
                None,
                &[],
                "main",
                i.1 .1
                    .iter()
                    .map(|x| x.clone().to_js_value_facade())
                    .collect::<Vec<_>>(),
            )?;
        }

        Ok(())
    }
}
