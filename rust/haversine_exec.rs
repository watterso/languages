extern crate geolocation;
extern crate haversine;

use std::env;
use haversine::Haversine;

fn main() {
    let args: Vec<_> = env::args().collect();
    let coords: Vec<f64> = args.into_iter()
                                .filter_map(|arg| arg.parse::<f64>().ok())
                                .collect();
    let start = geolocation::GeoLocationBuilder::new().lat(coords[0]).long(coords[1]).finalize();
    let end = geolocation::GeoLocationBuilder::new().lat(coords[2]).long(coords[3]).finalize();
    println!("start: {:?}\nend: {:?}", start, end);
    println!("distance: {} meters", start.haversine(&end));
}
