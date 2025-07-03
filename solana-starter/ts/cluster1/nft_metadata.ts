// import wallet from "/home/anish/.config/solana/id.json"
// import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
// import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
// import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
// import { readFile } from "fs/promises";

// // Create a devnet connection
// const umi = createUmi('https://api.devnet.solana.com');

// let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
// const signer = createSignerFromKeypair(umi, keypair);

// // umi.use(irysUploader());
// umi.use(irysUploader({address: "https://devnet.irys.xyz/",}));
// umi.use(signerIdentity(signer));

// (async () => {
//     try {
//         // Follow this JSON structure
//         // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

//         // const image = await readFile("./rug1.png");
//         const image = "https://gateway.irys.xyz/8Kt87aSW8btqyPTL92ewssJiFku5mASiWrPa2CM6VGs5";


//         const irysURI = "https://gateway.irys.xyz/8Kt87aSW8btqyPTL92ewssJiFku5mASiWrPa2CM6VGs5".replace(
//             "https://gateway.irys.xyz/",
//             "https://devnet.irys.xyz/"
//         )
//         console.log("your image URI: ", irysURI);

        
//         const metadata = {
//             name: "TestRUG1",
//             symbol: "X",
//             description: "turbin.funn",
//             image: image,
//             attributes: [
//                 {trait_type: 'colors', value: '20'}
//             ],
//             properties: {
//                 files: [
//                     {
//                         type: "image/png",
//                         uri: "image1"
//                     },
//                 ]
//             },
//             creators: []
//         };
//         const [myUri] = await umi.uploader.uploadJson(metadata);
//         console.log("Your metadata URI: ", myUri);
//     }
//     catch(error) {
//         console.log("Oops.. Something went wrong", error);
//     }
// })();

import wallet from "/home/anish/.config/solana/id.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/FwmSbw8qk2pNzEzYt62mgjKBuLAt8cAVcACHSnoUqUtQ"
        const metadata = {
            name: "ShinyShiny",
            symbol: "$",
            description: "Legendary Shinyshinyting From AnishSR",
            image: image,
            attributes: [
                {trait_type: 'Collection', value: 'Genesis'},
                {trait_type: 'Style', value: 'Modern'},
                {trait_type: 'Color', value: 'Brown'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        // upload metadata to the blockchain
        const metadataUri = await umi.uploader.uploadJson(metadata);
        const cleanedUrl = metadataUri.replace("https://arweave.net", "https://devnet.irys.xyz");
        console.log("Your metadata URI: ", cleanedUrl);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();