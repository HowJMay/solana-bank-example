import {
  establish_connection,
  establish_deposit_payer,
  establish_withdraw_payer,
  check_program,
  deposit_token,
  withdraw_token,
} from './client';

async function main() {
  console.log("A example of Solana Bank");

  // Establish connection to the cluster
  await establish_connection();

  // Determine who pays for the fees
  await establish_deposit_payer();
  await establish_withdraw_payer();

  // Check if the program has been deployed
  await check_program();

  // Deposit money
  await deposit_token();
  console.log('=======================================================');
  console.log('====================finish  deposit====================');
  console.log('=======================================================');

  await withdraw_token();
  console.log('=======================================================');
  console.log('====================finish withdraw====================');
  console.log('=======================================================');

  // // Demonstrate the recieved note
  // await report_received_note();

  console.log('SUCCESS');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
