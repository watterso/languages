haversine: libgeolocation.rlib
	rustc haversine.rs --extern geolocation=libgeolocation.rlib

haversine_test: haversine
	./haversine 37.773972 -122.431297 32.7833333 -96.8

libgeolocation.rlib:
	rustc --crate-type=lib geolocation.rs

clean:
	rm libgeolocation.rlib
	rm haversine
