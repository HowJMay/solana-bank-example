import {
  establish_connection,
  establish_payer,
  check_program,
  deposit_token,
  report_received_note,
} from './client';

async function main() {
  console.log("A example of Solana Bank");

  // Establish connection to the cluster
  await establish_connection();

  // Determine who pays for the fees
  await establish_payer();

  // Check if the program has been deployed
  await check_program();

  // Deposit money
  await deposit_token();

  // Demonstrate the recieved note
  await report_received_note();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
