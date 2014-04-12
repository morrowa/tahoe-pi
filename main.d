import std.conv, std.socket, std.stdio;

int main(char [][] args) {
	if (args.length != 3) {
		writeln("Usage: %s <host name> <host port>");
		return 1;
	}

	const ushort portNumber = to!ushort(args[2]);

	auto addresses = getAddress(args[1], portNumber);

	assert(addresses.length >= 1);

	Socket sock = new TcpSocket(addresses[0]);

	assert(sock.isAlive);

	sock.send("?WATCH={\"enable\":true,\"json\":true};\n");

	char[] buf = new char[1536];

	for (auto bytesRead = sock.receive(buf); bytesRead > 0; bytesRead = sock.receive(buf)) {
		writeln(buf[0..bytesRead]);
	}

	return 0;
}

