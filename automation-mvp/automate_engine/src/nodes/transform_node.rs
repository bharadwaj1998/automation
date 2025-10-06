use crate::nodes::Node;
use async_trait::async_trait;
use serde_json::{Value, json};
use handlebars::Handlebars;
use anyhow::{Result, anyhow};

pub struct TransformNode {
    handlebars: Handlebars<'static>,
}

impl TransformNode {
    /// Creates a new TransformNode instance with a handlebars renderer.
    pub fn new() -> Self {
        let mut h = Handlebars::new();
        // Optional: register a helper, if you ever need it later
        h.register_escape_fn(handlebars::no_escape);
        Self { handlebars: h }
    }
}

#[async_trait]
impl Node for TransformNode {
    /// Runs the transform node. The `template` config should be a Handlebars string.
    ///
    /// Example config:
    /// {
    ///   "template": "{\"summary\": \"{{body.current_user_url}}\"}"
    /// }
    async fn run(&self, config: &Value, input: Value) -> Result<Value> {
        // Get the template string
        let template = config.get("template")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("transform node missing 'template' field"))?;

        // Render using handlebars
        let rendered = self.handlebars.render_template(template, &input)?;

        // Try parsing rendered text into JSON (if valid)
        let parsed: Value = serde_json::from_str(&rendered).unwrap_or(json!(rendered));

        println!("🧠 Transform Node rendered output: {}", rendered);

        Ok(parsed)
    }
}
