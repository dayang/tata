use rocket_contrib::templates::handlebars;
use self::handlebars::{Helper, to_json, HelperDef, ScopedJson, RenderError, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
use pulldown_cmark::{Parser, html};
pub fn markdown_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let markdown_text = param.value().render();
        let parser = Parser::new(&markdown_text);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        out.write(&html_output)?;
    }

    Ok(())
}

pub struct NotEmptyStrHelper;

impl HelperDef for NotEmptyStrHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        //Ok(Some(ScopedJson::Derived(to_json(true))))
        Ok(Some(ScopedJson::Derived(to_json(h.param(0).unwrap().value().as_str().unwrap().len() > 0))))
    }
}