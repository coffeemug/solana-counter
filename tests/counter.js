const assert = require("assert");
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('counter', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  const program = anchor.workspace.Counter;
  const counterAccount = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.start(5, {
      accounts: {
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [counterAccount],
    });
  });

  it('Is incremented', async () => {
    const tx = await program.rpc.incr(5, {
      accounts: {
        counter: counterAccount.publicKey,
      },
    });

    const counter = await program.account.counter.fetch(counterAccount.publicKey);
    assert.equal(counter.value, 6);
  });
});
