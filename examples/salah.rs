use islam::pray::{error::Error, Config, Location, Madhab, Method, PrayerSchedule};
use time::{format_description, macros::date};

fn example() -> Result<(), Error> {
    // GMT+7
    let timezone = 7;
    // https://www.mapcoordinates.net/en
    let jakarta_city = Location::new(6.182_34_f32, 106.842_87_f32, timezone);
    let date = date!(2022 - 12 - 18);
    let config = Config::new().with(Method::Singapore, Madhab::Shafi);
    let prayer_times = PrayerSchedule::new(jakarta_city)?
        .on(date)
        .with_config(config)
        .calculate()?;

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

    let current_prayer = prayer_times.current()?;
    let (hour, minute) = prayer_times.time_remaining()?;
    println!("\nCurent Prayer");
    println!("{}: ({}:{})", current_prayer.name()?, hour, minute);

    let next_prayer = prayer_times.next()?;
    let time = prayer_times.time(next_prayer);
    let format = format_description::parse("[hour]:[minute]").unwrap();
    let time = time.format(&format).unwrap();

    println!("\nNext Prayer");
    println!("{}: ({})", next_prayer.name()?, time);

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        eprintln!("Error: {:?}", err);
    }
}
