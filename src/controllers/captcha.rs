use mini_captcha::*;
use rocket::response::content::Content;
use rocket::http::{ContentType, Cookies, Cookie};

#[get("/get")]
pub fn get_captcha(mut cookies: Cookies) -> Content<Vec<u8>> {
    let captcha = CaptchaBuilder::default()
        .width(80)
        .height(28)
        .chars(4)
        .build()
        .draw();

        let mut cookie = Cookie::new("code", captcha.captcha_code);
        cookie.set_expires(time::now_utc() + time::Duration::minutes(10));
        cookie.set_max_age(time::Duration::minutes(10));
        cookies.add_private(cookie);

    Content(ContentType::PNG, captcha.img_buffer)
}

