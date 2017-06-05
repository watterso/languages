haversine: libgeolocation.rlib
	rustc haversine.rs --extern geolocation=libgeolocation.rlib

libgeolocation.rlib:
	rustc --crate-type=lib geolocation.rs

clean:
	rm libgeolocation.rlib
	rm haversine
