var osmosis = require('osmosis');
var nodemailer = require('nodemailer');

var last_check = false;

check_and_mail()
setInterval(check_and_mail, 10*60*1000);

async function check_and_mail()
{
    let status = await check_status();

    if (status)
    {
        if (!last_check)
        {
            send_mail();
        }

        last_check = true;
    }
    else
    {
        last_check = false;
    }

    console.log((new Date()).toLocaleTimeString("en-us",
        {
            year: "numeric",
            month: "short",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit"
        }
    ) + ": " + (status ? "open" : "full"));
}

function send_mail()
{
    return new Promise(resolve => {
        let transporter = nodemailer.createTransport({
            host: 'smtp.gmail.com',
            port: 587,
            secure: false, // true for 465, false for other ports
            auth: {
                user: process.env.GMAIL_USERNAME,
                pass: process.env.GMAIL_PASSWORD
            }
        });

        let mailOptions = {
            from: 'bence.me@gmail.com',
            to: 'bence.me@gmail.com',
            cc: 'ghadeersammour@cmail.carleton.ca',
            subject: 'PHIL 2001 Notifier: Spot available',
            text: String.raw`There's a spot available in PHIL 2001.

Go here to check yourself: https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184189&crn=21329

Or this is wrong and you should tell me ¯\_(ツ)_/¯`
        };

        transporter.sendMail(mailOptions, (error, info) => {
            if (error)
            {
                return console.log(error);
            }

            resolve();
        });
    });
}

function check_status()
{
    return new Promise(resolve => {
        osmosis
        .get('https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184189&crn=21329')
        // .get('https://central.carleton.ca/prod/bwysched.p_display_course?wsea_code=EXT&term_code=201820&disp=8184189&crn=21327')
        .find('section > section > table > tr:nth-child(11)')
        .then(function(context, data, next)
        {
            data.status = context.innerText.replace(/\r\n|\r|\n/g, '');

            if (!data.status.match(/Status/))
            {
                next(context, data);
            }

            if (data.status.match(/Open/))
            {
                resolve(true);
            }
            else
            {
                resolve(false);
            }

            next(context, data);
        });
        // .log(console.log)
        // .error(console.log);
        // .debug(console.log);
    });
}
