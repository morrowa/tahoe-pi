package gps

import ("net"; "bufio"; "log"; "fmt"; "encoding/json")

type gpsdVersion struct {
	Class string
	Proto_major uint8
	Proto_minor uint8
}

type genericGPSDResponse struct {
	Class string
}

type TPVReport struct {
	Mode uint8
	Time string
	Lat float64
	Lon float64
	Alt float64
}

func Connect(host string) <-chan TPVReport {
	conn, err := net.Dial("tcp", host)
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

	return reports
}

func gpsdListen(conn *net.Conn, responses chan<- TPVReport) {
	lines := make(chan []byte)

	go readLines(conn, lines)

	var latestReport TPVReport

	for {
		select {
		case line := <-lines:
			var response genericGPSDResponse
			err := json.Unmarshal(line, &response)
			if err == nil && response.Class == "TPV" {
				var currentReport TPVReport
				err = json.Unmarshal(line, &currentReport)
				if err == nil {
					latestReport = currentReport
				}
			}
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

