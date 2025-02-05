// import {
//     Connection,
//     Keypair,
//     sendAndConfirmTransaction,
//     SystemProgram,
//     Transaction,
//     TransactionInstruction,
// } from '@solana/web3.js';
// import * as borsh from "borsh";
// import { Buffer } from "buffer";



// describe("Create a system account", async () => {

    // const connection = new Connection(`http://localhost:8899`, 'confirmed');
    // const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    // const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    // class Assignable {
    //     constructor(properties) {
    //         Object.keys(properties).map((key) => {
    //             return (this[key] = properties[key]);
    //         });
    //     };
    // };
    
    // class AddressData extends Assignable {
    //     toBuffer() {
    //         return Buffer.from(borsh.serialize(AddressDataSchema, this));
    //     }
    // };
    
    // const AddressDataSchema = new Map([
    //     [
    //         AddressData, {
    //             kind: 'struct',
    //             fields: [
    //                 ['name', 'string'],
    //                 ['address', 'string'],
    //             ]
    //         }
    //     ]
    // ]);
  
    // test("Create the account", async () => {

        // const newKeypair = Keypair.generate();

        // const addressData = new AddressData({
        //     name: "Marcus",
        //     address: "123 Main St. San Francisco, CA"
        // });

        // // We're just going to serialize our object here so we can check
        // //  the size on the client side against the program logs
        // //
        // const addressDataBuffer = addressData.toBuffer();
        // console.log(`Address data buffer length: ${addressDataBuffer.length}`)

        // let ix = new TransactionInstruction({
        //     keys: [
        //         {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        //         {pubkey: newKeypair.publicKey, isSigner: true, isWritable: true},
        //         {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
        //     ],
        //     programId: program.publicKey,
        //     data: addressDataBuffer,
        // });

        // await sendAndConfirmTransaction(
        //     connection, 
        //     new Transaction().add(ix),
        //     [payer, newKeypair]
        // );
//     });
//   });

import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import * as borsh from 'borsh';
import { readFileSync } from 'node:fs';
import { homedir } from 'node:os';

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(readFileSync(path, "utf-8")))
    )
};

describe('Create a system account', async () => {
    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/deploy/program-keypair.json');

    class Assignable {
        constructor(properties) {
            Object.keys(properties).map((key) => {
                return (this[key] = properties[key]);
            });
        };
    };
    
    class AddressData extends Assignable {
        toBuffer() {
            return Buffer.from(borsh.serialize(AddressDataSchema, this));
        }
    };
    
    const AddressDataSchema = new Map([
        [
            AddressData, {
                kind: 'struct',
                fields: [
                    ['name', 'string'],
                    ['address', 'string'],
                ]
            }
        ]
    ]);

  test('Create the account', async () => {
        const newKeypair = Keypair.generate();

        const addressData = new AddressData({
            name: "Marcus",
            address: "123 Main St. San Francisco, CA"
        });

        // We're just going to serialize our object here so we can check
        //  the size on the client side against the program logs
        //
        const addressDataBuffer = addressData.toBuffer();
        console.log(`Address data buffer length: ${addressDataBuffer.length}`)

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: newKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: program.publicKey,
            data: addressDataBuffer,
        });

        let signature = await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );

        let tx_data = await connection.getParsedTransaction(signature);
        console.log(tx_data.meta.logMessages);
  });
});