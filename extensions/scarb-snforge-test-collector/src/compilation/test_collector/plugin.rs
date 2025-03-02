use cairo_lang_defs::plugin::{MacroPlugin, PluginResult};
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_syntax::attribute::structured::AttributeListStructurize;
use cairo_lang_syntax::node::ast;
use cairo_lang_syntax::node::db::SyntaxGroup;

use super::forge_try_extract_test_config;

/// Plugin to create diagnostics for tests attributes.
#[derive(Debug, Default)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
struct TestPlugin;

impl MacroPlugin for TestPlugin {
    fn generate_code(&self, db: &dyn SyntaxGroup, item_ast: ast::ModuleItem) -> PluginResult {
        PluginResult {
            code: None,
            diagnostics: if let ast::ModuleItem::FreeFunction(free_func_ast) = item_ast {
                forge_try_extract_test_config(db, &free_func_ast.attributes(db).structurize(db))
                    .err()
            } else {
                None
            }
            .unwrap_or_default(),
            remove_original_item: false,
        }
    }

    fn declared_attributes(&self) -> Vec<String> {
        vec![
            "test".to_string(),
            "ignore".to_string(),
            "available_gas".to_string(),
            "should_panic".to_string(),
            "fork".to_string(),
            "fuzzer".to_string(),
        ]
    }
}

pub fn snforge_test_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_plugin::<TestPlugin>();
    suite
}
