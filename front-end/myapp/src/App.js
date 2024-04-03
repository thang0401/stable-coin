import React, {useState} from 'react';
import logo from './logo.svg';
import './App.css';
import * as anchor from '@project-serum/anchor'; 
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import idl from './my-app.json'

// systemProgram is a reference to the solana runtime!
const {SystemProgram, Keypair} = anchor.web3;

// create a keypair for account that will hold the variable data.
// let myAccount = Keypair.generate();

// get our program's id from IDL file 
// const programID = new PublicKey(idl.metadata.address)
// console.log(programID, 'program ID set correctly')

//set our network to devnet
// const network = clusterApiUrl('devnet');

// controls how we want to acknowledge when a transaction is "done";
// const opts ={
//   preflightCommitment : 'processed',
// }
function App() {
  const [walletAddress, setWalletAddress] = useState(null);
  window.onload = async function(){
    try {
      if(window.solana){
        //code here
        const solana = window.solana 
        if(solana.isPhantom){
          console.log ('Phantom wallet found! ')
          const res = await solana.connect({onlyIfTrusted: true})
          console.log('connected with Public Key: ', res.publicKey.toString())
        }
      }else {
        alert ('wallet not found!')
      }

    }catch (error) {
      console.error(error);
    }
  }
  const connectwallet = async() => {
    if(window.solana){
        const solana = window.solana 
        const res = await solana.connect()
        setWalletAddress(res.publicKey.toString())
        console.log('connected with Public Key: ', res.publicKey.toString())
    }else {
      alert('wallet not found!')
    }
  }

  // const getProvider = () => {
  //   const connection = new Connection(network, opts.preflightCommitment)
  //   const provider = new anchor.AnchorProvider(
  //     connection,
  //     window.solana,
  //     opts.preflightCommitment,
  //     )
  //     console.log(provider,'provider set correctly')
  //     return provider
  // }
  return (
    <div className="App">
      <header className="App-header">
        {!walletAddress && (
          <div>
            <button className='btn' onClick={connectwallet}>
              Connect Wallet
            </button>
          </div>
        ) }
        {walletAddress && (
          <div>
            <button className='btn' onClick={connectwallet}>
              Connect Wallet
            </button>
          </div>
        ) }
        {walletAddress && (
          <div>
            <p>
              connect account :{' '}
              <span className='address'> {walletAddress}</span>
            </p>
          </div>
        ) }
      </header>
    </div>
  );
}

export default App;
