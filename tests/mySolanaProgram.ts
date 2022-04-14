import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MySolanaProgram } from '../target/types/my_solana_program';
import { expect, assert } from 'chai';

describe('mySolanaProgram', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MySolanaProgram as Program<MySolanaProgram>;

  it('setup tweet platform!', async () => {
    // Add your test here.
    const tweetKeypair = anchor.web3.Keypair.generate();
    const user = program.provider.wallet;
    await program.rpc.setupPlatform({
      accounts: {
        tweet: tweetKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [tweetKeypair]
    });
    
    let tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal('');
  });
});
