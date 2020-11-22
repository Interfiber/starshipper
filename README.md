# 🌌🚀 Starshipper
Transfer files over your local network

## Using

To run starshipper on your local network you will need to clone this repo somewhere on your local machine. You should also have mongodb installed(If you want to use the account database), and the latest version of rust. Then cd into the server folder and start the backend with
```console
node --no-warnings main.js
```
Then in a new tab run
```console
mkdir data && mongod --dbpath data
```
make sure this is also in the server folder. You will also need to configure you local computer addess(Example: cool-computer.local) to set your computer address set the LOCAL_COMPUTER varible in your shell to it. Now to use the frontend you can compile the CLI with
```console
cargo build
```
Now you can run the executable generated(Located at ./target/debug/starshipper). Now your ready to go! To upload a file you can run
```console
./target/debug/starshipper send
```
Then enter the filename that you wish to upload. Anything that is binary(Like zipfiles) cannot be uploaed yet, along with directorys. Then after uploading it will give you a UUID. To download a file go to another computer(Make sure it is connected to the same network) and compile the Commandline interface on it and then run
```console
./target/debug/starshipper retrive
```
Then enter the UUID, and then enter the file you wish to save the file to. Then The file should be transfered!
