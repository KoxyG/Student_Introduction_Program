import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai"
import { StudentIntro } from "../target/types/student_intro";

describe("student_intro", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)
 

  const program = anchor.workspace.StudentIntro as Program<StudentIntro>;


  const student = {
    name: "Alice",
    message: "I am a computer science student",
  }

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Is introduction!", async () => {
    // Add your test here.
    const tx = await program.methods.introduce(student.name, student.message).rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.studentAccountState.fetch(studentPda)


    expect(account.name).to.equal(student.name)
    expect(account.message).to.equal(student.message)
    expect(account.introducer.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
  });


  it("Student introduction updated", async () => {
    const newName = "New name now"
    const newMessage = "New message now"

    const tx = await program.methods.updateIntroduce(newName, newMessage ).rpc()

    const account = await program.account.studentAccountState.fetch(studentPda)

    expect(account.name).to.equal(newName)
    expect(account.message).to.equal(newMessage)
    expect(account.introducer.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
  })


  it('Deletes an introduction', async () => {
    const tx = await program.methods.deleteStudentIntro(student.name).rpc()
  })

});
