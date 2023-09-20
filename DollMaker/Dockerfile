# Description: This image is used to develop and run all executables in the project.
# This docker environment will support the following:
# - bash scripts
# - python scripts (python3)
# - compiled C/C++ program using glibc for linux
# - Javascript and Typescript scripts
# - C# executables (.NET Core 7 and ASP.NET Core 7)
# - Rust executables

# ubuntu latest -> replace sh with bash -> update and upgrade packages
FROM ubuntu:latest
# use bash instead of sh
SHELL ["/bin/bash", "-c"]
RUN rm /bin/sh && ln -s /bin/bash /bin/sh
RUN apt update && apt upgrade -y
# languages already supported by the base image: bash, C, C++
# for C#, Rust, Javascript, Typescript, Python we need to install the required packages:
# - .NET Core 7
# - NodeJS
# - Rust
# - Python3

# install .NET SDK
RUN apt install -y dotnet-sdk-7.0
# install NodeJS by first installing nvm, then install latest lts version of node, then use it, then check the version
RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
RUN source ~/.nvm/nvm.sh && nvm install --lts && nvm use --lts && node -v
# install Rust
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y
# install Python3
RUN apt install -y python3

# install git
RUN apt install -y git
# install vscode server
RUN curl -fsSL https://code-server.dev/install.sh | sh

# make a directory for the project and set it as the working directory
RUN mkdir /DollMaker && cd /DollMaker

# define environment variables
ENV TEST_VAR="test"

# run indefinitely to prevent the container from exiting
CMD sleep infinity