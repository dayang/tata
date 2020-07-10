use multipart::server::Multipart;
use rocket_contrib::json::JsonValue;

use rocket::http::ContentType;
use rocket::request::State;
use rocket::Data;

use super::super::{JsonErr, User};

use crate::qiniu::{upload_file, Qiniu, UploadType};

#[post("/image/upload", data = "<data>")]
pub fn upload_image(
    _user: User,
    cont_type: &ContentType,
    data: Data,
    qiniu: State<Qiniu>,
) -> JsonValue {
    if !cont_type.is_form_data() {
        return json!(JsonErr::Err("Content-Type not multipart/form-data".into()));
    }

    if let Some((_, boundary)) = cont_type.params().find(|&(k, _)| k == "boundary") {
        match Multipart::with_body(data.open(), boundary).read_entry() {
            Ok(Some(multipart_field)) => {
                if multipart_field.headers.name.as_ref() == "file" {
                    let file_name = multipart_field.headers.filename.unwrap_or("temp".into());
                    json!(
                        upload_file(multipart_field.data, file_name, &qiniu, UploadType::Image)
                            .map(|v| format!("{}/{}", qiniu.host, v))
                    )
                } else {
                    json!(JsonErr::Err("no file field".into()))
                }
            }
            _ => json!(JsonErr::Err("server read data failed".into())),
        }
    } else {
        json!(JsonErr::Err(
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        ))
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![upload_image,]
}
