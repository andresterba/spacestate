use std::time;

use spacestate;

fn main() -> Result<(), reqwest::Error> {
    let mut config = spacestate::Config::new();
    config.get_from_env();

    let five_seconds = time::Duration::from_secs(config.fetch_interval);
    let mut last_observed_status: spacestate::RoomStatus = spacestate::RoomStatus::Closed;

    loop {
        let room_status = spacestate::fetch_room_status(&config.spaceapi_url);

        if room_status == last_observed_status {
            println!("no change in room_status; will not notify");
            std::thread::sleep(five_seconds);
            continue;
        }

        last_observed_status = room_status;

        let send_status = spacestate::send_room_status(&config.webhook_url, room_status);
        match send_status {
            Ok(_) => std::thread::sleep(five_seconds),
            Err(e) => println!("{e:?}"),
        }

        std::thread::sleep(five_seconds);
    }

    // Ok(())
}
