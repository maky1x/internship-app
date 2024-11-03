use std::{fs::File, io::{ErrorKind, Read, Write}};

use rocket::{data::ToByteUnit, fairing::{Fairing, Info, Kind}, http::{ContentType, Header}, launch, options, post, routes, Data, Request, Response};
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};


#[post("/upload", data = "<data>")]

async fn upload(content_type: &ContentType, data: Data <'_>)
-> Result<String, std::io::Error> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("image")
        .size_limit(u64::from(32.mebibytes()))
    ]);

    let multi_form_data = MultipartFormData::parse(content_type, data, options)
    .await
    .unwrap();

    let file = multi_form_data.files.get("image");

    println!();

    if let Some(file_fields) = file {
        let file_field = &file_fields[0];

        let filename = &file_field.file_name;
        let content_type = &file_field.content_type;

        println!("FileName: {:?}", filename);
        println!("content_type: {:?}", content_type);

        if let Some(mime) = content_type {
            if mime.to_string() != "image/png" {
                return Err(std::io::Error::new(ErrorKind::Other, "Only PNG images are supported."))
            }
        }

        let mut file = File::create(format!("../../../internship-app-front/public/{}", filename.clone().unwrap()))?;

        let path = &file_field.path;

        let mut temp_file = File::open(path)?;

        let mut buffer = Vec::new();

        temp_file.read_to_end(&mut buffer)?;
        file.write_all(&buffer)?;

        return Ok("File processed".into());
    }

    Err(std::io::Error::new(ErrorKind::Other, "Upload failed!"))
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[options("/upload")]
fn all_options() {

}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    .mount("/", routes![upload, all_options])
}


