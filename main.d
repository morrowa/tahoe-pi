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

	char[] buf = new char[1536];

	auto bytesRead = sock.receive(buf);

	if (bytesRead > 0) {
		writeln(buf[0..bytesRead]);
	}

	return 0;
}

