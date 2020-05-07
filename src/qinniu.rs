use rocket::fairing::AdHoc;
use rocket::Rocket;

pub struct Qiniu {
    pub host: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String
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
        let bucket = qiniu_config
            .get("bucket")
            .expect("missing qiniu:bucket config")
            .as_str()
            .expect("bucket should be string")
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

        Ok(rocket.manage(Qiniu{
            host,
            bucket,
            access_key,
            secret_key,
        }))
    }))
}
