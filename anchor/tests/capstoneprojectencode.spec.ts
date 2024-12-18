import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {Capstoneprojectencode} from '../target/types/capstoneprojectencode'

describe('capstoneprojectencode', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Capstoneprojectencode as Program<Capstoneprojectencode>

  const capstoneprojectencodeKeypair = Keypair.generate()

  // it('Initialize Capstoneprojectencode', async () => {
  //   await program.methods
  //     .initialize()
  //     .accounts({
  //       capstoneprojectencode: capstoneprojectencodeKeypair.publicKey,
  //       payer: payer.publicKey,
  //     })
  //     .signers([capstoneprojectencodeKeypair])
  //     .rpc()

  //   const currentCount = await program.account.capstoneprojectencode.fetch(capstoneprojectencodeKeypair.publicKey)

  //   expect(currentCount.count).toEqual(0)
  // })

  it('Fetch Validators', async () => {
    await program.methods.fetchValidators().accounts({
      voteAccounts: "your_vote_account_address",
    }).rpc();
  
    // Check logs or outputs after calling fetchValidators
    // For example, you can check the state of the validator's node_pubkey:
    const voteState = await program.account.voteState.fetch("your_vote_account_address");
    expect(voteState.nodePubkey).toBeDefined();
  });

  // it('Set close the capstoneprojectencode account', async () => {
  //   await program.methods
  //     .close()
  //     .accounts({
  //       payer: payer.publicKey,
  //       capstoneprojectencode: capstoneprojectencodeKeypair.publicKey,
  //     })
  //     .rpc()

  //   // The account should no longer exist, returning null.
  //   const userAccount = await program.account.capstoneprojectencode.fetchNullable(capstoneprojectencodeKeypair.publicKey)
  //   expect(userAccount).toBeNull()
  // })
})
