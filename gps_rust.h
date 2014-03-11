// gps_rust.h
// Simplified interface to gpsd

typedef void* gps_rust_client_data;

struct gps_rust_fix {
	int mode;
	double time;
	double latitude;
	double longitude;
};

gps_rust_client_data gps_rust_open(void);

int gps_rust_read(gps_rust_client_data, struct gps_rust_fix*);

void gps_rust_close(gps_rust_client_data);

