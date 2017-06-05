#[derive(Debug)]
pub struct GeoLocation {
    latitude: f64,
    longitude: f64
}

#[derive(Debug)]
pub struct GeoLocationInRadians {
    pub latitude: f64,
    pub longitude: f64
}

impl GeoLocation {
    pub fn to_radians(&self) -> GeoLocationInRadians {
        GeoLocationInRadians{
            latitude: self.latitude.to_radians(),
            longitude: self.longitude.to_radians()
        }
    }
}

pub struct GeoLocationBuilder{
    lat: f64,
    long: f64
}

fn valid_latitude(angle: f64) -> bool { angle >= -90.0f64 && angle <= 90.0f64 }
fn valid_longitude(angle: f64) -> bool { angle >= -180.0f64 && angle <= 180.0f64 }

impl GeoLocationBuilder {
    pub fn new() -> GeoLocationBuilder {
        GeoLocationBuilder { lat: 0.0, long: 0.0 }
    }

    pub fn lat(&mut self, new_lat: f64) -> &mut GeoLocationBuilder {
        self.lat = new_lat;
        self
    }

    pub fn long(&mut self, new_long: f64) -> &mut GeoLocationBuilder {
        self.long = new_long;
        self
    }

    pub fn finalize(&self) -> GeoLocation {
        if valid_latitude(self.lat) && valid_longitude(self.long){
            GeoLocation{ latitude: self.lat, longitude: self.long }
        } else{
            panic!("invalid lat-long: {}, {}", self.lat, self.long);
        }
    }
}
