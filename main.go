package main

import ("fmt"; "time"; "github.com/morrowa/tahoe-pi/gps"; "github.com/morrowa/tahoe-pi/stn1110")

func main() {

	_ := stn1110.Connect("/dev/null")

	reports := gps.Connect("127.0.0.1:2947")

	for {
		pos := <-reports
		fmt.Printf("Lat: %f\n", pos.Lat)
		fmt.Printf("Lon: %f\n", pos.Lon)
		time.Sleep(time.Second)
	}
}

