How to build the connection-profiler.  

Linux:
```shell
sudo yum -y install gcc git 
curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
git clone https://github.com/fritshoogland-yugabyte/connection-profiler.git
cd connection-profiler
cargo build --release
```
Mac OSX:
```shell
curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
git clone https://github.com/fritshoogland-yugabyte/connection-profiler.git
cd connection-profiler
cargo build --release
```
