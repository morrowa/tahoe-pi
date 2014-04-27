package main

import ("net"; "bufio"; "log"; "fmt"; "encoding/json")

type gpsdVersion struct {
	Class string
	Proto_major uint8
	Proto_minor uint8
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
	} else {
		fmt.Printf("GPSD Version: %d.%d\n", version.Proto_major, version.Proto_minor)
	}
}

