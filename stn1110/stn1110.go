package stn1110

type STNCommandMode string

const (
	AT  STNCommandMode = "AT"
	ST  STNCommandMode = "ST"
	OBD STNCommandMode = ""
)

type STNCommand struct {
	Mode STNCommandMode
	//...
}

type STNResponse struct {
	//...
}

func Connect(devPath string) (commandChan chan<- STNCommand, responseChan <-chan STNResponse) {
	//...
}

