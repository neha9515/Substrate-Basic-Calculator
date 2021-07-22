# Substrate-Basic-Calculator
The project Calculator is developed in the substrate to perform simple arithmetic calculations.

## Installation
You will need to do some setup to prepare your system for substrate development. Simply go to [substrate.dev](https://substrate.dev/) and follow the installation instruction.

## Add a Pallet
This project contains one calculator pallet. In that, we perform arithmetic operations such as addition, subtraction, multiplication, and division.

## Run Locally
To add your pallet, Substrate-Node-Template/pallets

#### Clone the project

 > git clone -b v3.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-pallet-template calculator
  
#### Go to the pallet

 > cd calculator
  
#### To add more dependencies just go through [substrate-pallet-template](https://github.com/substrate-developer-hub/substrate-pallet-template/blob/master/README.md)

Once you added all dependencies run the below commands in the root directory

 > cargo build --release 
  
After the build succeeds, you can start the node

 > ./target/release/node-template --dev --tmp
 
Once your node is running, you will notice that blocks are being produced. At that time go to the [polkadot.js](https://polkadot.js.org/apps/#/explorer)

#### Click on, Developer->extrinsics->calculator

The fields can be filled like this,
![cal1](https://user-images.githubusercontent.com/85206495/126631674-8510e4d7-b863-4efe-b700-a63889c304e1.png)

#### Click on, Network->explorer
![cal](https://user-images.githubusercontent.com/85206495/126631723-996b20f8-6ff1-4f3b-8259-ad68717ee9c9.png)

