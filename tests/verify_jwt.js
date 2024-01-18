import * as anchor from "@coral-xyz/anchor";
import {createAndSendV0Tx} from "./utils/tx.js"
import {expect} from "./utils/solana-chai.js";

describe("Verify JWT", () => {  
  it("should verify", async () => {
    const provider = anchor.AnchorProvider.local();
    const program = anchor.workspace.ZkinSmartwallet;
    const user = provider.wallet.payer;
  
    const header = Buffer.from("eyJhbGciOiJSUzI1NiIsImtpZCI6IjQ4YTYzYmM0NzY3Zjg1NTBhNTMyZGM2MzBjZjdlYjQ5ZmYzOTdlN2MiLCJ0eXAiOiJKV1QifQ", "base64");
    const payload = Buffer.from("eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiI0ODMyMTI1NDgwNzgtcWNoMWdzdnNsc3J1cTMyMWgybDNiaGVhcDluYTFvZzMuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiI0ODMyMTI1NDgwNzgtcWNoMWdzdnNsc3J1cTMyMWgybDNiaGVhcDluYTFvZzMuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTczNjM5MTkxNzM0Mzc1NzU4MzAiLCJlbWFpbCI6InBwNmcxMUBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwibm9uY2UiOiI4NjlhZWYwODU4ZWJlZmIxOGI0YzVhNDBjY2MyYWY2YTljY2RhZjE4ZTU1OGY2OGIzZGMyMTUxOGIyNDJkYjZlIiwibmJmIjoxNzA1NDA4NzMwLCJuYW1lIjoicGF2bG9zIHBhcGEiLCJwaWN0dXJlIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSmY0M2tCbm5oRW9nWHhIZzA3dkJDZDhEYkNUT1NzVGRQa0ZTNTE4aDVOPXM5Ni1jIiwiZ2l2ZW5fbmFtZSI6InBhdmxvcyIsImZhbWlseV9uYW1lIjoicGFwYSIsImxvY2FsZSI6ImVuIiwiaWF0IjoxNzA1NDA5MDMwLCJleHAiOjE3MDU0MTI2MzAsImp0aSI6IjhkNDAwN2ZhZGM0ZWM5ODA3ZWQwNDI0ZDY2ODlhMTc0YTVlYzAzZWQifQ", "base64");
    const sig = Buffer.from("WImgfozLUGEzlIbN_SSbQ4WQ0yQZnnWURoYpfN1HjKs2hWeoHAVz_CyfHAK2bDdRN8QfYSufz6t7_3Ck5ZgNCGJuAWjIITAEdnxHT8nEo1Z19e1k301phLXjzWaptMtmtUw_WqoKnchQOxSR5LsEfcvdWlirnCZAuiWXQ1JqHXB7IFgDq_t7Q_LgxN8YmVaAWXJ92jPkjfNPWf61B_zQ_OG5VlYkNPSKNfj90-69v2cMker5GQ_Z_WwSMMLkm_izkl8gpFj9jVFJEtzsyFV27kej5QkV-GeejsQPOjnG0-ULbFSgvuN0qekOabR1Tgftk-DMkduNFt-QMhp8S4Xddg", "base64");

    const ix = await program.methods
    .verifyJwt(header, payload, sig)
    .accounts({
      user: user.publicKey,
    })
    .instruction();
  
    await createAndSendV0Tx(
      provider,
      [ix],
      user.publicKey,
      [user]
    );
  })
});

