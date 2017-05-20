use std::env;

#[derive(Debug)]
struct GeoLocation {
    latitude: f64,
    longitude: f64
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

}
