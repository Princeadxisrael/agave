// #![allow(clippy::arithmetic_side_effects)]
// // #![feature(test)]

<<<<<<< HEAD
// use {
//     crossbeam_channel::{unbounded, Receiver},
//     rayon::{
//         iter::IndexedParallelIterator,
//         prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
//     },
//     solana_core::banking_stage::{
//         committer::Committer, consumer::Consumer, qos_service::QosService,
//     },
//     solana_entry::entry::Entry,
//     solana_ledger::{
//         blockstore::Blockstore,
//         genesis_utils::{create_genesis_config, GenesisConfigInfo},
//     },
//     solana_poh::{
//         poh_recorder::{create_test_recorder, PohRecorder},
//         poh_service::PohService,
//     },
//     solana_runtime::{bank::Bank, bank_forks::BankForks},
//     solana_sdk::{
//         account::{Account, ReadableAccount},
//         signature::Keypair,
//         signer::Signer,
//         stake_history::Epoch,
//         system_program, system_transaction,
//         transaction::SanitizedTransaction,
//     },
//     std::sync::{
//         atomic::{AtomicBool, Ordering},
//         Arc, RwLock,
//     },
//     tempfile::TempDir,
//     test::Bencher,
// };
=======
use {
    crossbeam_channel::{unbounded, Receiver},
    rayon::{
        iter::IndexedParallelIterator,
        prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    },
    solana_core::banking_stage::{
        committer::Committer, consumer::Consumer, qos_service::QosService,
    },
    solana_entry::entry::Entry,
    solana_ledger::{
        blockstore::Blockstore,
        genesis_utils::{create_genesis_config, GenesisConfigInfo},
    },
    solana_poh::{
        poh_recorder::create_test_recorder, poh_service::PohService,
        transaction_recorder::TransactionRecorder,
    },
    solana_runtime::{bank::Bank, bank_forks::BankForks},
    solana_runtime_transaction::runtime_transaction::RuntimeTransaction,
    solana_sdk::{
        account::{Account, ReadableAccount},
        signature::Keypair,
        signer::Signer,
        stake_history::Epoch,
        system_program, system_transaction,
        transaction::SanitizedTransaction,
    },
    std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    tempfile::TempDir,
    test::Bencher,
};
>>>>>>> upstream/master

// extern crate test;

// fn create_accounts(num: usize) -> Vec<Keypair> {
//     (0..num).into_par_iter().map(|_| Keypair::new()).collect()
// }

// fn create_funded_accounts(bank: &Bank, num: usize) -> Vec<Keypair> {
//     assert!(
//         num.is_power_of_two(),
//         "must be power of 2 for parallel funding tree"
//     );
//     let accounts = create_accounts(num);

//     accounts.par_iter().for_each(|account| {
//         bank.store_account(
//             &account.pubkey(),
//             &Account {
//                 lamports: 5100,
//                 data: vec![],
//                 owner: system_program::id(),
//                 executable: false,
//                 rent_epoch: Epoch::MAX,
//             }
//             .to_account_shared_data(),
//         );
//     });

//     accounts
// }

<<<<<<< HEAD
// fn create_transactions(bank: &Bank, num: usize) -> Vec<SanitizedTransaction> {
//     let funded_accounts = create_funded_accounts(bank, 2 * num);
//     funded_accounts
//         .into_par_iter()
//         .chunks(2)
//         .map(|chunk| {
//             let from = &chunk[0];
//             let to = &chunk[1];
//             system_transaction::transfer(from, &to.pubkey(), 1, bank.last_blockhash())
//         })
//         .map(SanitizedTransaction::from_transaction_for_tests)
//         .collect()
// }

// fn create_consumer(poh_recorder: &RwLock<PohRecorder>) -> Consumer {
//     let (replay_vote_sender, _replay_vote_receiver) = unbounded();
//     let committer = Committer::new(None, replay_vote_sender, Arc::default());
//     let transaction_recorder = poh_recorder.read().unwrap().new_recorder();
//     Consumer::new(committer, transaction_recorder, QosService::new(0), None)
// }

// struct BenchFrame {
//     bank: Arc<Bank>,
//     _bank_forks: Arc<RwLock<BankForks>>,
//     ledger_path: TempDir,
//     exit: Arc<AtomicBool>,
//     poh_recorder: Arc<RwLock<PohRecorder>>,
//     poh_service: PohService,
//     signal_receiver: Receiver<(Arc<Bank>, (Entry, u64))>,
// }
=======
fn create_transactions(bank: &Bank, num: usize) -> Vec<RuntimeTransaction<SanitizedTransaction>> {
    let funded_accounts = create_funded_accounts(bank, 2 * num);
    funded_accounts
        .into_par_iter()
        .chunks(2)
        .map(|chunk| {
            let from = &chunk[0];
            let to = &chunk[1];
            system_transaction::transfer(from, &to.pubkey(), 1, bank.last_blockhash())
        })
        .map(RuntimeTransaction::from_transaction_for_tests)
        .collect()
}

fn create_consumer(transaction_recorder: TransactionRecorder) -> Consumer {
    let (replay_vote_sender, _replay_vote_receiver) = unbounded();
    let committer = Committer::new(None, replay_vote_sender, Arc::default());
    Consumer::new(committer, transaction_recorder, QosService::new(0), None)
}

struct BenchFrame {
    bank: Arc<Bank>,
    _bank_forks: Arc<RwLock<BankForks>>,
    ledger_path: TempDir,
    exit: Arc<AtomicBool>,
    transaction_recorder: TransactionRecorder,
    poh_service: PohService,
    signal_receiver: Receiver<(Arc<Bank>, (Entry, u64))>,
}
>>>>>>> upstream/master

// fn setup() -> BenchFrame {
//     let mint_total = u64::MAX;
//     let GenesisConfigInfo {
//         mut genesis_config, ..
//     } = create_genesis_config(mint_total);

//     // Set a high ticks_per_slot so we don't run out of ticks
//     // during the benchmark
//     genesis_config.ticks_per_slot = 10_000;

//     let mut bank = Bank::new_for_benches(&genesis_config);

//     // Allow arbitrary transaction processing time for the purposes of this bench
//     bank.ns_per_slot = u128::MAX;

//     // set cost tracker limits to MAX so it will not filter out TXs
//     bank.write_cost_tracker()
//         .unwrap()
//         .set_limits(u64::MAX, u64::MAX, u64::MAX);
//     let (bank, bank_forks) = bank.wrap_with_bank_forks_for_tests();

<<<<<<< HEAD
//     let ledger_path = TempDir::new().unwrap();
//     let blockstore = Arc::new(
//         Blockstore::open(ledger_path.path()).expect("Expected to be able to open database ledger"),
//     );
//     let (exit, poh_recorder, poh_service, signal_receiver) =
//         create_test_recorder(bank.clone(), blockstore, None, None);

//     BenchFrame {
//         bank,
//         _bank_forks: bank_forks,
//         ledger_path,
//         exit,
//         poh_recorder,
//         poh_service,
//         signal_receiver,
//     }
// }
=======
    let ledger_path = TempDir::new().unwrap();
    let blockstore = Arc::new(
        Blockstore::open(ledger_path.path()).expect("Expected to be able to open database ledger"),
    );
    let (exit, _poh_recorder, transaction_recorder, poh_service, signal_receiver) =
        create_test_recorder(bank.clone(), blockstore, None, None);

    BenchFrame {
        bank,
        _bank_forks: bank_forks,
        ledger_path,
        exit,
        transaction_recorder,
        poh_service,
        signal_receiver,
    }
}
>>>>>>> upstream/master

// fn bench_process_and_record_transactions(bencher: &mut Bencher, batch_size: usize) {
//     const TRANSACTIONS_PER_ITERATION: usize = 64;
//     assert_eq!(
//         TRANSACTIONS_PER_ITERATION % batch_size,
//         0,
//         "batch_size must be a factor of `TRANSACTIONS_PER_ITERATION` \
//          ({TRANSACTIONS_PER_ITERATION}) so that bench results are easily comparable"
//     );
//     let batches_per_iteration = TRANSACTIONS_PER_ITERATION / batch_size;

<<<<<<< HEAD
//     let BenchFrame {
//         bank,
//         _bank_forks,
//         ledger_path: _ledger_path,
//         exit,
//         poh_recorder,
//         poh_service,
//         signal_receiver: _signal_receiver,
//     } = setup();
//     let consumer = create_consumer(&poh_recorder);
//     let transactions = create_transactions(&bank, 2_usize.pow(20));
//     let mut transaction_iter = transactions.chunks(batch_size);

//     bencher.iter(move || {
//         for _ in 0..batches_per_iteration {
//             let summary = consumer.process_and_record_transactions(
//                 &bank,
//                 transaction_iter.next().unwrap(),
//                 0,
//             );
//             assert!(summary
//                 .execute_and_commit_transactions_output
//                 .commit_transactions_result
//                 .is_ok());
//         }
//     });
=======
    let BenchFrame {
        bank,
        _bank_forks,
        ledger_path: _ledger_path,
        exit,
        transaction_recorder,
        poh_service,
        signal_receiver: _signal_receiver,
    } = setup();
    let consumer = create_consumer(transaction_recorder);
    let transactions = create_transactions(&bank, 2_usize.pow(20));
    let mut transaction_iter = transactions.chunks(batch_size);

    bencher.iter(move || {
        for _ in 0..batches_per_iteration {
            let summary =
                consumer.process_and_record_transactions(&bank, transaction_iter.next().unwrap());
            assert!(summary
                .execute_and_commit_transactions_output
                .commit_transactions_result
                .is_ok());
        }
    });
>>>>>>> upstream/master

//     exit.store(true, Ordering::Relaxed);
//     poh_service.join().unwrap();
// }

// #[bench]
// fn bench_process_and_record_transactions_unbatched(bencher: &mut Bencher) {
//     bench_process_and_record_transactions(bencher, 1);
// }

// #[bench]
// fn bench_process_and_record_transactions_half_batch(bencher: &mut Bencher) {
//     bench_process_and_record_transactions(bencher, 32);
// }

// #[bench]
// fn bench_process_and_record_transactions_full_batch(bencher: &mut Bencher) {
//     bench_process_and_record_transactions(bencher, 64);
// }
