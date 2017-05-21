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

#[derive(Debug)]
struct GeoLocation {
    latitude: f64,
    longitude: f64
}

#[derive(Debug)]
struct GeoLocationInRadians {
    latitude: f64,
    longitude: f64
}

fn haversine(point1: GeoLocationInRadians, point2: GeoLocationInRadians) -> f64 {
    haversine_f64(
        point1.latitude - point2.latitude,
        point1.latitude,
        point2.latitude,
        point1.longitude - point2.longitude
    )
}

impl GeoLocation {
    fn to_radians(&self) -> GeoLocationInRadians {
        GeoLocationInRadians{
            latitude: self.latitude.to_radians(),
            longitude: self.longitude.to_radians()
        }
    }

    fn haversine(&self, other: &GeoLocation) -> f64 {
        haversine(self.to_radians(), other.to_radians())
    }
}

struct GeoLocationBuilder{
    lat: f64,
    long: f64
}

trait ValidCoordinate {
    fn valid_latitude(&self) -> bool;
    fn valid_longitude(&self) -> bool;
}

impl ValidCoordinate for f64{
    fn valid_latitude(&self) -> bool { *self >= -90.0f64 && *self <= 90.0f64 }
    fn valid_longitude(&self) -> bool { *self >= -180.0f64 && *self <= 180.0f64 }
}

impl GeoLocationBuilder {
    fn new() -> GeoLocationBuilder {
        GeoLocationBuilder { lat: 0.0, long: 0.0 }
    }

    fn lat(&mut self, new_lat: f64) -> &mut GeoLocationBuilder {
        self.lat = new_lat;
        self
    }

    fn long(&mut self, new_long: f64) -> &mut GeoLocationBuilder {
        self.long = new_long;
        self
    }

    fn finalize(&self) -> GeoLocation {
        if self.lat.valid_latitude() && self.long.valid_longitude(){
            GeoLocation{ latitude: self.lat, longitude: self.long }
        } else{
            panic!("invalid lat-long: {}, {}", self.lat, self.long);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let coords: Vec<f64> = args.into_iter()
                                .filter_map(|arg| arg.parse::<f64>().ok())
                                .collect();
    let start = GeoLocationBuilder::new().lat(coords[0]).long(coords[1]).finalize();
    let end = GeoLocationBuilder::new().lat(coords[2]).long(coords[3]).finalize();
    println!("start: {:?}\nend: {:?}", start, end);
    println!("distance: {} meters", start.haversine(&end));

}
