use rocket_contrib::templates::handlebars;
use self::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
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