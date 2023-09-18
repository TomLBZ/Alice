# Description: This image is used to develop and run all executables in the project.
# This docker environment will support the following:
# - bash scripts
# - python scripts (python3)
# - compiled C/C++ program using glibc for linux
# - Javascript and Typescript scripts
# - C# executables (.NET Core 7 and ASP.NET Core 7)
# - Rust executables

# the base image will be ubuntu. The latest version is 22.04
FROM ubuntu:latest
# make sure the image is up to date
RUN sudo apt update && sudo apt upgrade -y
# some languages are already supported by the base image: bash, python3, C, C++
# for other languages (C#, Rust, Javascript, Typescript), we need to install the required packages:
# - .NET Core 7
# - NodeJS
# - Rust

# install .NET SDK
RUN sudo apt install -y dotnet-sdk-7.0
# install NodeJS by first installing nvm, then install latest lts version of node, then use it, then check the version
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash && source ~/.bashrc && nvm install --lts && nvm use --lts && node -v
# install Rust
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y