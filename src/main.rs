extern crate reqwest;
extern crate scraper;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate chrono;

use lettre::{smtp, EmailTransport, ClientTlsParameters, ClientSecurity};

use native_tls::TlsConnector;
use native_tls::{Protocol};

fn main()
{
    let mut tls_builder = TlsConnector::builder().unwrap();
    tls_builder.supported_protocols(&[Protocol::Tlsv10]).unwrap();
    let tls_parameters = ClientTlsParameters::new(
        "smtp.gmail.com".to_string(),
        tls_builder.build().unwrap()
    );

    println!("Using username \"{}\"", std::env::var("GMAIL_USERNAME").expect("Could not get environment variable GMAIL_USERNAME"));
    println!("Using password \"{}\"", std::env::var("GMAIL_PASSWORD").expect("Could not get environment variable GMAIL_PASSWORD"));

    let mut mailer = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", 587), ClientSecurity::Opportunistic(tls_parameters))
        .expect("Failed to create transport")
        .authentication_mechanism(lettre::smtp::authentication::Mechanism::Login)
        .credentials(lettre::smtp::authentication::Credentials::new(
            std::env::var("GMAIL_USERNAME").expect("Could not get environment variable GMAIL_USERNAME"),
            std::env::var("GMAIL_PASSWORD").expect("Could not get environment variable GMAIL_PASSWORD")
        ))
        .build();

    let mut last_check = false;

    loop
    {
        // let html = reqwest::get("https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184239&crn=21327")
        let html = reqwest::get("https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184189&crn=21329")
            .expect("Failed to request PHIL course page")
            .text()
            .expect("Failed to convert request into text");

        let document = scraper::Html::parse_document(&html);

        let row = document.select(&scraper::Selector::parse("body > section > section > table > tbody > tr:nth-child(11)").unwrap())
            .next().expect("Failed to find table row");

        let status_str = row.text().collect::<Vec<&str>>().join("").replace("\n", "");

        if !status_str.contains("Status:") { panic!("Retrieved row doesn't contain the word 'status'") }

        if status_str.contains("Open")
        {
            if !last_check
            {
                let email = lettre_email::EmailBuilder::new()
                    .from("bence.me@gmail.com")
                    .to("bence.me@gmail.com")
                    .subject("PHIL 2001 Notifier: Spot available")
                    .body(r#"There's a spot available in PHIL 2001.

Go here to check yourself: https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184189&crn=21329"#)
                    .build()
                    .expect("Failed to build email");
                
                mailer.send(&email).expect("Failed to send success email.");
            }

            last_check = true;
        }
        else
        {
            last_check = false;
        }
        // status_str.contains("Full") {}


        println!("{}: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), status_str);

        std::thread::sleep(std::time::Duration::from_secs(10*30));
    }
}