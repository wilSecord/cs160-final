use std::{io::{BufReader, BufRead}, fs::File, error::Error, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>>{
    let states: HashMap<usize, (usize, String)> = BufReader::new(File::open("../data/artist_states.csv")?).lines()
    .skip(1).filter_map(|line| {
        let line = line.ok()?;
        let [artist, state, guid] = line.split(',').collect::<Vec<_>>()[..] else { return None };

        Some((artist.parse().unwrap(), (state.parse().unwrap(), guid.to_string())))
    }).collect();

    let longlats: HashMap<String, (f64, f64)> = BufReader::new(File::open("../results/mbid-longlat.csv")?).lines()
    .skip(1).filter_map(|line| {
        let line = line.ok()?;
        let [guid, lat, lon] = line.split(',').collect::<Vec<_>>()[..] else { return None };

        Some((guid.to_string(), (lat.parse().unwrap(), lon.parse().unwrap())))
    }).collect();


    for (artist, (state, guid)) in states {
        if let Some((lat,lon)) = longlats.get(&guid) {
            println!("'{artist}':[{lat},{lon}],")
        }
    }


    Ok(())
}