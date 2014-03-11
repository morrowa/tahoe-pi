// gps_rust.c
// Simplified interface to gpsd

#include "gps_rust.h"
#include <stdlib.h>
#include <gps.h>


gps_rust_client_data gps_rust_open(void) {
	struct gps_data_t *newclient = malloc(sizeof(struct gps_data_t));
	if (newclient == NULL) {
		return NULL;
	}

	int returnCode = gps_open("shared memory", NULL, newclient);
	if (returnCode != 0) {
		free(newclient);
		return NULL;
	}

	return newclient;
}

int gps_rust_read(gps_rust_client_data clientdata, struct gps_rust_fix* outfix) {
	struct gps_data_t *realClientData = clientdata;
	int returncode = gps_read(realClientData);
	if (returncode >= 0) {
		outfix->mode = realClientData->fix.mode;
		outfix->time = realClientData->fix.time;
		outfix->latitude = realClientData->fix.latitude;
		outfix->longitude = realClientData->fix.longitude;
	}

	return returncode;
}

void gps_rust_close(gps_rust_client_data clientdata) {
	gps_close((struct gps_data_t*)clientdata);
}

