# Alice
A simple framework for quick development and rapid iteration of microservices in any language.
## The Vision
No networking, no boilerplate, no fuss. You only have to build a program that reads from stdin and writes to stdout. The framework will take care of the rest.
## How to use
### Developers
#### Server Side
- Clone the repository and run the install script.
- Run `alice shelf up` to start the server. Arguments are:
    | Argument | Description |
    | --- | --- |
    | -p, --port | The port on which the server should listen for. Default is `9090`. |
    | -h, --help | Show the help message. |
- To stop the server, run `alice shelf down`.
#### Dev PC
- Clone the repository and run the install script.
- Develop your program that reads from stdin and writes to stdout. Produce an executable.
- Run `alice doll <name> <path-to-your-program>` to start your executable and register it with the server by your given name. Arguments are:
    | Argument | Description |
    | --- | --- |
    | -s, --server | The address and port of the server. Default is `localhost:9090`. |
    | -p, --port | The port on which the server is listening. Default is 9090. |
    | -h, --help | Show the help message. |