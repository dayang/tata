use self::handlebars::{
    to_json, Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output,
    RenderContext, RenderError, ScopedJson,
};

use crate::dto::book::CatalogItem;
use pulldown_cmark::{html, Options, Parser};
use rocket_contrib::templates::handlebars;

const ALLOWED_CODE_CLASSES: &[&'static str] = &[
    "language-bash",
    "language-clike",
    "language-go",
    "language-ini",
    "language-javascript",
    "language-java",
    "language-json",
    "language-markup",
    "language-protobuf",
    "language-ruby",
    "language-rust",
    "language-scss",
    "language-sql",
    "language-csharp",
    "language-cs",
    "language-golang",
    "language-shell",
    "language-html",
    "language-css",
    "language-cpp",
    "language-kotlin",
    "language-php",
    "language-python",
    "language-r",
    "language-matlab",
    "language-scala",
    "language-yaml",
    "language-lua",
    "language-groovy",
    "language-typescript",
];

pub fn markdown_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let markdown_text = param.value().render();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&markdown_text, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let cleaned = ammonia::Builder::default()
            .add_allowed_classes("code", ALLOWED_CODE_CLASSES)
            .clean(&html_output)
            .to_string();
        // let safe_html = clean(&*html_output);
        // out.write(&safe_html)?;
        out.write(&cleaned)?;
    }

    Ok(())
}

pub fn comment_type_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let comment_type = param.value().as_i64();
        let label = match comment_type {
            Some(1) => "博客",
            Some(2) => "Book",
            Some(3) => "留言",
            _ => "未知",
        };

        out.write(label)?;
    }

    Ok(())
}

fn print_catalog_item(out: &mut dyn Output, item: CatalogItem, level: i32) -> HelperResult {
    out.write(&format!(
        "<div class=\"level-{}\"><a href=\"/books/page/{}\">{} <span class=\"create-time layui-hide-xs layui-hide-sm layui-show-md-block\">{}</span></a></div>",
        level, item.url, item.title, item.create_time
    ))?;
    for child in item.children {
        print_catalog_item(out, child, level + 1)?;
    }

    Ok(())
}

pub fn book_catalog_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let catalog_items: Vec<CatalogItem> =
            serde_json::from_value(param.value().clone()).unwrap();
        for item in catalog_items {
            print_catalog_item(out, item, 1)?
        }
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
        Ok(Some(ScopedJson::Derived(to_json(
            h.param(0).unwrap().value().as_str().unwrap().len() > 0,
        ))))
    }
}
