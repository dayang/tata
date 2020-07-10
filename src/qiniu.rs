use rocket::fairing::AdHoc;
use rocket::Rocket;
use std::io::Read;

use crate::service::err_str;
use qiniu_ng::{
    storage::upload_policy::UploadPolicyBuilder, storage::upload_token::UploadToken,
    storage::uploader::UploadManager, ConfigBuilder, Credential,
};
use std::time::Duration;

#[derive(Clone)]
pub struct Qiniu {
    pub host: String,
    pub image_bucket: String,
    pub db_bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

pub enum UploadType {
    DB,
    Image,
}

pub fn attach_qiniu(rocket: Rocket) -> Rocket {
    rocket.attach(AdHoc::on_attach("Init Qiniu", |rocket| {
        let qiniu_config = rocket
            .config()
            .get_table("qiniu")
            .expect("missing qiniu config");

        let host = qiniu_config
            .get("host")
            .expect("missing qiniu:host config")
            .as_str()
            .expect("host should be string")
            .to_string();
        let image_bucket = qiniu_config
            .get("image_bucket")
            .expect("missing qiniu:image_bucket config")
            .as_str()
            .expect("image_bucket should be string")
            .to_string();
        let db_bucket = qiniu_config
            .get("db_bucket")
            .expect("missing qiniu:db_bucket config")
            .as_str()
            .expect("db_bucket should be string")
            .to_string();
        let access_key = qiniu_config
            .get("access_key")
            .expect("missing qiniu:access_key config")
            .as_str()
            .expect("access_key should be string")
            .to_string();

        let secret_key = qiniu_config
            .get("secret_key")
            .expect("missing qiniu:secret_key config")
            .as_str()
            .expect("secret_key should be string")
            .to_string();

        Ok(rocket.manage(Qiniu {
            host,
            image_bucket,
            db_bucket,
            access_key,
            secret_key,
        }))
    }))
}

pub fn upload_file<T: Read + Send>(
    stream: T,
    file_name: String,
    qiniu: &Qiniu,
    upload_type: UploadType,
) -> Result<String, String> {
    let credential = Credential::new(qiniu.access_key.clone(), qiniu.secret_key.clone());
    let config = ConfigBuilder::default().build();
    let bucket_name = match upload_type {
        UploadType::DB => &qiniu.db_bucket,
        UploadType::Image => &qiniu.image_bucket,
    };
    let upload_policy =
        UploadPolicyBuilder::new_policy_for_bucket(bucket_name.clone(), Duration::from_secs(7200))
            .build();

    let upload_token = UploadToken::from_policy(upload_policy, credential);
    let upload_manager = UploadManager::new(config);
    let upload_response = upload_manager
        .for_upload_token(upload_token)
        .map_err(err_str)?
        .upload_stream(stream, Some(file_name), None)
        .map_err(err_str)?;
    upload_response
        .key()
        .map(|k| k.to_string())
        .ok_or("no key in qiniu api response".into())
}
