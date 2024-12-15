// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import CapstoneprojectencodeIDL from '../target/idl/capstoneprojectencode.json'
import type { Capstoneprojectencode } from '../target/types/capstoneprojectencode'

// Re-export the generated IDL and type
export { Capstoneprojectencode, CapstoneprojectencodeIDL }

// The programId is imported from the program IDL.
export const CAPSTONEPROJECTENCODE_PROGRAM_ID = new PublicKey(CapstoneprojectencodeIDL.address)

// This is a helper function to get the Capstoneprojectencode Anchor program.
export function getCapstoneprojectencodeProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...CapstoneprojectencodeIDL, address: address ? address.toBase58() : CapstoneprojectencodeIDL.address } as Capstoneprojectencode, provider)
}

// This is a helper function to get the program ID for the Capstoneprojectencode program depending on the cluster.
export function getCapstoneprojectencodeProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Capstoneprojectencode program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return CAPSTONEPROJECTENCODE_PROGRAM_ID
  }
}
