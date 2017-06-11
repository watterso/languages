extern crate geolocation;

use std::env;
use std::f64;

fn course12(
    point1: &geolocation::GeoLocationInRadians,
    point2: &geolocation::GeoLocationInRadians,
    dst12: f64
) -> f64 {
    let numerator = point2.latitude.sin()-point1.latitude.sin()*dst12.cos(); 
    let denominator = dst12.sin()*point1.latitude.cos();
    //println!("crs12 {} / {} = {}", numerator, denominator,  (numerator / denominator).acos());
    (numerator / denominator).acos()
}

fn course21(
    point1: &geolocation::GeoLocationInRadians,
    point2: &geolocation::GeoLocationInRadians,
    dst12: f64
) -> f64 {
    let numerator = point1.latitude.sin()-point2.latitude.sin()*dst12.cos(); 
    let denominator = dst12.sin()*point2.latitude.cos();
    //println!("crs21 {} / {} = {}", numerator, denominator,  (numerator / denominator).acos());
    (numerator / denominator).acos()
}


fn intersection(
    point1: &geolocation::GeoLocation,
    crs13: geolocation::Course,
    point2: &geolocation::GeoLocation,
    crs23: geolocation::Course
) -> geolocation::GeoLocation {
    let point1_rad = point1.to_radians();
    let point2_rad = point2.to_radians();
    let lon1 = point1_rad.longitude;
    let lon2 = point2_rad.longitude;
    let dst12 = 2f64*(
        (
            (((point1_rad.latitude-point2_rad.latitude)/2f64).sin()).powi(2) + 
            point1_rad.latitude.cos()*point2_rad.latitude.cos()*(((lon1-lon2)/2f64).sin()).powi(2)
        ).sqrt()
    ).asin();
    //println!("point1_rad: {:?}\npoint2_rad: {:?}", point1_rad, point2_rad);
    //println!("dst12: {}", dst12);
    //TODO find out what this condition actually means
    //println!("cond: {}", (point2_rad.longitude - point1_rad.longitude).sin());
    let (crs12, crs21) =
        if (point2_rad.longitude - point1_rad.longitude).sin() < 0f64 {
            (course12(&point1_rad, &point2_rad, dst12), 2f64 * f64::consts::PI - course21(&point1_rad, &point2_rad, dst12))
        } else {
            (2f64 * f64::consts::PI - course12(&point1_rad, &point2_rad, dst12), course21(&point1_rad, &point2_rad, dst12))
        };
    let crs12 = course12(&point1_rad, &point2_rad, dst12);
    let crs21 = course21(&point1_rad, &point2_rad, dst12);
    //println!("crs12: {:?}\ncrs21: {:?}", crs12, crs21);
    //TODO modulo vs remainder sign stuff
    let ang1 = crs13-crs12;
    let ang2 = crs21-crs23;

    //TODO do a `Result` thing
    if ang1.sin() == 0f64 && ang2.sin() == 0f64 {
        //"infinite intersections" ?they on top of each other?
        panic!("infinity of intersections"); 
    } else if ang1.sin() * ang2.sin() == 0f64 {
        //"intersection ambiguous" ?parallel?
        panic!("intersection ambiguous");
    }
    let mut result_builder = geolocation::GeoLocationBuilder::new();

    let ang1 = ang1.abs();
    let ang2 = ang2.abs();
    //acos(-cos(ang1)*cos(ang2)+sin(ang1)*sin(ang2)*cos(dst12)) 
    let ang3 = (-ang1.cos()*ang2.cos()+ang1.sin()*ang2.sin()*dst12.cos()).acos();
    //atan2(sin(dst12)*sin(ang1)*sin(ang2),cos(ang2)+cos(ang1)*cos(ang3))
    let dst13 = (dst12.sin()*ang1.sin()*ang2.sin()).atan2(ang2.cos()+ang1.cos()*ang3.cos());
    //asin(sin(lat1)*cos(dst13)+cos(lat1)*sin(dst13)*cos(crs13))
    let lat3 =
        (point1_rad.latitude.sin()*dst13.cos()+point1_rad.latitude.cos()*dst13.sin()*crs13.cos()).asin();
    result_builder.lat(lat3.to_degrees());
    //atan2(sin(crs13)*sin(dst13)*cos(lat1),cos(dst13)-sin(lat1)*sin(lat3))
    let dlon =
        (crs13.sin()*dst13.sin()*point1_rad.latitude.cos()).atan2(dst13.cos()-point1_rad.latitude.sin()*lat3.sin());
    let lon3 = point1_rad.longitude+dlon;
    //println!("ang1: {:?}\nang2: {:?}\nang3: {:?}\ndst13: {:?}\nlat3,lon3: {},{}\n", ang1, ang2, ang3, dst13, lat3,lon3);
    result_builder.long(lon3.to_degrees());
    result_builder.finalize()
}


fn main() {
    let args: Vec<_> = env::args().collect();
    let coords: Vec<f64> = args.into_iter()
                                .filter_map(|arg| arg.parse::<f64>().ok())
                                .collect();
    let point1 = geolocation::GeoLocationBuilder::new().lat(coords[0]).long(coords[1]).finalize();
    let course1: geolocation::Course = coords[2];
    let point2 = geolocation::GeoLocationBuilder::new().lat(coords[3]).long(coords[4]).finalize();
    let course2: geolocation::Course = coords[5];
    println!("point1: {:?}, course1: {:?}\npoint2: {:?}, course2: {:?}", point1, course1, point2,
             course2);
    let res = intersection(&point1, course1.to_radians(), &point2, course2.to_radians());
    println!("intersects at: {:?}", res);
}
