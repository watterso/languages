extern crate geolocation;

use std::env;

//a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
//c = 2 ⋅ atan2( √a, √(1−a) )
//d = R ⋅ c
//where φ is latitude, λ is longitude, R is earth’s radius (mean radius = 6,371km);
//note that angles need to be in radians to pass to trig functions!
fn haversine_f64(delta_lat: f64, lat1: f64, lat2: f64, delta_long:f64) -> f64 {
    let a = (delta_lat / 2.0f64).sin().powi(2) + lat1.cos() * lat2.cos() * (delta_long / 2.0f64).sin().powi(2);
    let c = 2.0f64 * a.sqrt().atan2((1.0f64-a).sqrt());
    let radius_earth_meters = 6371000.0f64;
    radius_earth_meters * c
}


fn haversine(point1: geolocation::GeoLocationInRadians, point2: geolocation::GeoLocationInRadians) -> f64 {
    haversine_f64(
        point1.latitude - point2.latitude,
        point1.latitude,
        point2.latitude,
        point1.longitude - point2.longitude
    )
}

trait Haversine<T> {
    fn haversine(&self, other: &T) -> f64;
}

impl Haversine<geolocation::GeoLocation> for geolocation::GeoLocation {
    fn haversine(&self, other: &geolocation::GeoLocation) -> f64 {
        haversine(self.to_radians(), other.to_radians())
    }
}


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
