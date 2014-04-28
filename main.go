package main

import ("net"; "bufio"; "log"; "fmt"; "encoding/json")

type gpsdVersion struct {
	Class string
	Proto_major uint8
	Proto_minor uint8
}

type TPVReport struct {
	Mode uint8
	Time string
	Lat float64
	Lon float64
	Alt float64
}

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:2947")
	if err != nil {
		log.Fatal(err)
	}

	reader := bufio.NewReader(conn)

	hello, err := reader.ReadBytes('\n')
	if err != nil {
		log.Fatal(err)
	}

	var version gpsdVersion
	err = json.Unmarshal(hello, &version)
	if err != nil {
		log.Fatal(err)
	} else if version.Proto_major != 3 {
		log.Fatal("Only version 3.x of the gpsd protocol is supported (found version %d.%d)", version.Proto_major, version.Proto_minor)
	}

	_, err = fmt.Fprintln(conn, "?WATCH={\"enable\": true, \"json\": true}")
	if err != nil {
		log.Fatal(err)
	}

	reports := make(chan TPVReport)

	go gpsdListen(&conn, reports)

	sync := make(chan bool)
	<-sync
}

func gpsdListen(conn *net.Conn, responses chan<- TPVReport) {
	lines := make(chan []byte)

	go readLines(conn, lines)

	var latestReport TPVReport

	for {
		select {
		case line := <-lines:
			print(line)
		case responses <- latestReport:
		}
	}
}

func readLines(conn *net.Conn, lines chan<- []byte) {
	reader := bufio.NewReader(*conn)

	for {
		buff, err := reader.ReadBytes('\n')
		if err != nil {
			log.Fatal(err)
		}

		lines <- buff
	}
}

