use reqwest::blocking::*;
use reqwest::header::*;
use reqwest::Error;



pub fn get_metadata(ytid: String) -> Result<Response, Error>{
    let client = Client::new();
    client.post("https://beatsage.com/youtube_metadata")
    .header(ACCEPT, "*/*")
    .header(USER_AGENT, "Overcomplicating-Things/0.1.0")
    .header(CONTENT_TYPE, "application/json; charset=utf-8")
    .header(HOST, "beatsage.com")
    .header(EXPECT, "100-continue")
    .body("{\"youtube_url\":\"https://www.youtube.com/watch?v=".to_owned() + &ytid + "\"}")
    .send()
}

pub fn get_yt_cover(ytid: String) -> Result<Response, Error>{
    let client = Client::new();
    client.get("https://img.youtube.com/vi/".to_owned() + &ytid + "/hqdefault.jpg")
    .send()
}


pub unsafe fn submit(ytid: String, cover: Vec<u8>, title: String, artist: String) -> Result<Response, Error> {


    let url = "https://www.youtube.com/watch?v=".to_owned() + &ytid;
    dbg!(url.clone());

    let boundary = "------WebKitFormBoundaryaA38RFcmCeKFPOms\r\n";
    let body =
    boundary.to_owned() + 
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=youtube_url\r\n\r\n" + &url + "\r\n"
    
    + boundary +
    "Content-Disposition: form-data; name=\"cover_art\"; filename=\"cover\"" + "\r\n" +
    "Content-Type: image/jpeg\r\n\r\n";
    
    let mut body = body.as_bytes().to_vec();

    let mut file = cover.clone();

    body.append(&mut file);
    
    let remain
    = "\r\n".to_owned() + boundary + 
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=audio_metadata_title\r\n\r\n" + &title.replace("\x20\x2F\x2F\x20", "\x20\x5F\x20") + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=audio_metadata_artist\r\n\r\n" + &artist + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=difficulties\r\n\r\n" + "Normal" + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=modes\r\n\r\n" + "Standard" + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=events\r\n\r\n" + "" + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=environment\r\n\r\n" + "DefaultEnvironment" + "\r\n"

    + boundary +
    "Content-Type: text/plain; charset=utf-8\r\n" + 
    "Content-Disposition: form-data; name=system_tag\r\n\r\n" + "v2" + "\r\n"

    + boundary + "\x2D\x2D"
    ;
    
    let mut remain = remain.as_bytes().to_vec();

    body.append( &mut remain );

    // Last four bytes are in the wrong order
    let byte1 = body.pop().unwrap();
    let byte2 = body.pop().unwrap();
    let byte3 = body.pop().unwrap();
    let byte4 = body.pop().unwrap();

    let mut sig = vec![byte1, byte2, byte4, byte3];
    body.append(&mut sig);


    let client = Client::new();
    client.post("https://beatsage.com/beatsaber_custom_level_create")
    .header("Accept", "*/*")
    .header("User-Agent", "Overcomplicating-Things/0.1.0")
    .header("Content-Type", "multipart/form-data; boundary=\"----WebKitFormBoundaryaA38RFcmCeKFPOms\"")
    .header("Host", "beatsage.com")
    .header("Expect", "100-continue")
    .body(body)
    .send()
}


pub fn check(job_id: String) -> Result<Response, Error> {
    let client = Client::new();
    client.get("https://beatsage.com/beatsaber_custom_level_heartbeat/".to_owned() + &job_id)
    .header(ACCEPT, "*/*")
    .header(USER_AGENT, "Overcomplicating-Things/0.1.0")
    .header(HOST, "beatsage.com")
    .send()
}

pub fn download(job_id: String) -> Result<Response, Error> {
    let client = Client::new();
    client.get("https://beatsage.com/beatsaber_custom_level_download/".to_owned() + &job_id)
    .header(HOST, "beatsage.com")
    .send()
}



/////////////////

use serde::*;
use std::{thread, time::Duration};
use base64::{Engine as _, engine::{general_purpose}};

#[derive(Serialize, Deserialize)]
struct Metadata {
    original_url: String,
    beatsage_thumbnail: String,
    title: String,
    channel: String,
}

#[derive(Serialize, Deserialize)]
struct ID {
    id: String,
}


#[derive(Serialize, Deserialize, Clone)]
struct Status {
    status: String,
}


pub fn submit_and_download(ytid: String, title: String) -> Result<Response, Error>{
    unsafe {
            let x = ytid.to_string();
        
            let raw = get_metadata(x.clone()).unwrap().text().unwrap();
            let md: Metadata = serde_json::from_str(&raw).unwrap();
        
            let cover_bytes = general_purpose::STANDARD.decode(md.beatsage_thumbnail).unwrap();
        
            
            let raw_id = submit(x.clone(), cover_bytes, title, md.channel).unwrap();
        
            let resp_text = raw_id.text().unwrap();
        
            dbg!(resp_text.clone());
        
            let job_id: ID = serde_json::from_str(&resp_text).unwrap();
            let job_id = job_id.id;
        
            dbg!(job_id.clone());
        
        
            loop {
                thread::sleep(Duration::from_millis(4000));
                let job_status = check(job_id.clone());
                let job_status = &job_status.unwrap().text().unwrap();
                let status: Status = serde_json::from_str(job_status).unwrap();
                dbg!(status.clone().status);
                if status.clone().status == "PENDING" {
                    println!("not done");
                } else {
                    println!("done");
                    break;
                }
            }
        
            let dl = download(job_id);
            return dl;
    }
}