use chrono::prelude::*;
use islam::pray::config::Config;
use islam::pray::madhab::Madhab;
use islam::pray::method::Method;
use islam::pray::times::{Location, PrayerSchedule};

fn main() {
    // GMT+7
    let timezone = 7;
    // https://www.mapcoordinates.net/en
    let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
    let date = Local.ymd(2021, 4, 9);
    let config = Config::new().with(Method::Singapore, Madhab::Shafi);
    let prayer_times = PrayerSchedule::new(jakarta_city)
        .on(date)
        .with_config(config)
        .calculate();

    let fajr = prayer_times.fajr;
    println!("fajr: {}:{}:{}", fajr.hour(), fajr.minute(), fajr.second());

    let sherook = prayer_times.sherook;
    println!(
        "sherook: {}:{}:{}",
        sherook.hour(),
        sherook.minute(),
        sherook.second()
    );

    let dohr = prayer_times.dohr;
    println!("dohr: {}:{}:{}", dohr.hour(), dohr.minute(), dohr.second());

    let asr = prayer_times.asr;
    println!("asr: {}:{}:{}", asr.hour(), asr.minute(), asr.second());

    let maghreb = prayer_times.maghreb;
    println!(
        "maghreb: {}:{}:{}",
        maghreb.hour(),
        maghreb.minute(),
        maghreb.second()
    );

    let ishaa = prayer_times.ishaa;
    println!(
        "ishaa: {}:{}:{}",
        ishaa.hour(),
        ishaa.minute(),
        ishaa.second()
    );
}
