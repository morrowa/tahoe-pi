package main

import ("fmt"; "time"; "github.com/morrowa/tahoe-pi/gps")

func main() {

	reports := gps.Connect("127.0.0.1:2947")

	for {
		pos := <-reports
		fmt.Printf("Lat: %f\n", pos.Lat)
		fmt.Printf("Lon: %f\n", pos.Lon)
		time.Sleep(time.Second)
	}
}

