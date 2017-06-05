haversine_exec: libhaversine.rlib
	rustc haversine_exec.rs --extern geolocation=libgeolocation.rlib --extern haversine=libhaversine.rlib

intersection: libgeolocation.rlib
	rustc intersection.rs --extern geolocation=libgeolocation.rlib

haversine_test: haversine_exec
	./haversine_exec 37.773972 -122.431297 32.7833333 -96.8

intersection_test: intersection
	./intersection 42.600 -117.866 51 44.840 -117.806 137

libgeolocation.rlib:
	rustc --crate-type=lib geolocation.rs

libhaversine.rlib: libgeolocation.rlib
	rustc --crate-type=lib haversine.rs --extern geolocation=libgeolocation.rlib

clean:
	rm -f *.rlib
	rm -f haversine_exec
	rm -f intersection
