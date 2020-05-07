use rocket_contrib::json::{ JsonValue};
use multipart::server::Multipart;

use rocket::Data;
use rocket::http::{ContentType};
use rocket::request::State;

use super::super::{User, JsonErr};
use crate::service::err_str;
use std::io::Read;

use qiniu_ng::{
    Credential, ConfigBuilder,
    storage::uploader::UploadManager,
    storage::upload_token::UploadToken,
    storage::upload_policy::UploadPolicyBuilder
};
use std::time::Duration;
use crate::qinniu::Qiniu;

fn upload_qiniu<T: Read + Send>(stream: T, file_name: String, qiniu: &Qiniu) -> Result<String, String> {
    let credential = Credential::new(qiniu.access_key.clone(), qiniu.secret_key.clone());
    let config = ConfigBuilder::default().build();
    let upload_policy = UploadPolicyBuilder::new_policy_for_bucket(qiniu.bucket.clone(), Duration::from_secs(7200))
                                            .build();
    
    let upload_token = UploadToken::from_policy(upload_policy, credential);
    let upload_manager = UploadManager::new(config);
    let upload_response = upload_manager.for_upload_token(upload_token).map_err(err_str)?
        .upload_stream(stream, Some(file_name), None).map_err(err_str)?;
    upload_response.key().map(|v| format!("{}/{}", qiniu.host,v)).ok_or("no key in qiniu api response".into())
}

#[post("/image/upload", data="<data>")]
pub fn upload_image(_user: User, cont_type: &ContentType, data: Data, qiniu: State<Qiniu>) -> JsonValue {
    if !cont_type.is_form_data() {
        return json!(JsonErr::Err("Content-Type not multipart/form-data".into()));
    }

    if let Some((_, boundary)) = cont_type.params().find(|&(k, _)| k == "boundary") {
        match Multipart::with_body(data.open(), boundary).read_entry() {
            Ok(Some(multipart_field)) => {
                if multipart_field.headers.name.as_ref() == "file" {
                    let file_name = multipart_field.headers.filename.unwrap_or("temp".into());
                    json!(upload_qiniu(multipart_field.data, file_name, &qiniu))
                } else {
                    json!(JsonErr::Err("no file field".into()))
                }
            }
            _ => json!(JsonErr::Err("server read data failed".into()))
        }
    } else {
        json!(JsonErr::Err("`Content-Type: multipart/form-data` boundary param not provided".into()))
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        upload_image,
    ]
}
