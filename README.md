# Alice
A simple tool for quick development and rapid iteration of microservices in any language.
## The Vision
No networking, no boilerplate, no fuss. You only have to build a program that reads from stdin and writes to stdout. The tool will take care of the rest. Make sure your program or script is executable. If it is a script, add a shebang line at the top. If it is a compiled binary, make sure it is in your server's PATH. You can use the sample service `dep.py` for installing dependencies if your service requires them. You can find it under the `samples/svc` directory.
## How to use
### Developers
- Clone the repository and run `docker compose up`.
- Use the webpage GUI to create a new service, etc. at `http://localhost:18000`.
- Recommended to create a new service with the name `dep` and `dep.py` as the executable in `Service` mode. This service will be used to install dependencies for other services.