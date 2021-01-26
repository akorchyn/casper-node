use casper_engine_test_support::{
    internal::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder,
        DEFAULT_RUN_GENESIS_REQUEST,
    },
    AccountHash, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use casper_execution_engine::storage::protocol_data::DEFAULT_WASMLESS_TRANSFER_COST;
use casper_types::{mint, runtime_args, RuntimeArgs, U512};

const ACCOUNT_1_ADDR: AccountHash = AccountHash::new([1u8; 32]);

#[ignore]
#[test]
fn ee_1160_wasmless_transfer_should_empty_account() {
    let transfer_amount =
        U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE) - U512::from(DEFAULT_WASMLESS_TRANSFER_COST);

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let default_account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should get default_account");

    let no_wasm_transfer_request_1 = {
        let wasmless_transfer_args = runtime_args! {
            mint::ARG_TARGET => ACCOUNT_1_ADDR,
            mint::ARG_AMOUNT => transfer_amount,
            mint::ARG_ID => <Option<u64>>::None
        };

        let deploy_item = DeployItemBuilder::new()
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_empty_payment_bytes(runtime_args! {})
            .with_transfer_args(wasmless_transfer_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .build();
        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    };

    builder
        .exec(no_wasm_transfer_request_1)
        .expect_success()
        .commit();

    let last_result = builder.get_exec_result(0).unwrap().clone();
    let last_result = &last_result[0];

    assert!(last_result.as_error().is_none(), "{:?}", last_result);
    assert!(!last_result.transfers().is_empty());

    let default_account_balance_after = builder.get_purse_balance(default_account.main_purse());

    let account_1 = builder
        .get_account(ACCOUNT_1_ADDR)
        .expect("should get default_account");
    let account_1_balance = builder.get_purse_balance(account_1.main_purse());

    assert_eq!(default_account_balance_after, U512::zero());
    assert_eq!(account_1_balance, transfer_amount);
}

#[ignore]
#[test]
fn ee_1160_transfer_larger_than_balance_should_fail() {
    let transfer_amount = U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)
        - U512::from(DEFAULT_WASMLESS_TRANSFER_COST)
        // One above the available balance to transfer should raise an InsufficientPayment already
        + U512::one();

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let default_account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should get default_account");

    let balance_before = builder.get_purse_balance(default_account.main_purse());

    let no_wasm_transfer_request_1 = {
        let wasmless_transfer_args = runtime_args! {
            mint::ARG_TARGET => ACCOUNT_1_ADDR,
            mint::ARG_AMOUNT => transfer_amount,
            mint::ARG_ID => <Option<u64>>::None
        };

        let deploy_item = DeployItemBuilder::new()
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_empty_payment_bytes(runtime_args! {})
            .with_transfer_args(wasmless_transfer_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .build();
        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    };

    builder.exec(no_wasm_transfer_request_1).commit();

    let balance_after = builder.get_purse_balance(default_account.main_purse());

    assert_eq!(balance_after, balance_before,);

    let last_result = builder.get_exec_result(0).unwrap().clone();
    let last_result = &last_result[0];

    assert!(
        last_result.as_error().is_some(),
        "Expected error but last result is {:?}",
        last_result
    );
    assert!(
        last_result.transfers().is_empty(),
        "Expected empty list of transfers"
    );
}

#[ignore]
#[test]
fn ee_1160_large_wasmless_transfer_should_avoid_overflow() {
    let transfer_amount = U512::max_value();

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let default_account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should get default_account");

    let balance_before = builder.get_purse_balance(default_account.main_purse());

    let no_wasm_transfer_request_1 = {
        let wasmless_transfer_args = runtime_args! {
            mint::ARG_TARGET => ACCOUNT_1_ADDR,
            mint::ARG_AMOUNT => transfer_amount,
            mint::ARG_ID => <Option<u64>>::None
        };

        let deploy_item = DeployItemBuilder::new()
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_empty_payment_bytes(runtime_args! {})
            .with_transfer_args(wasmless_transfer_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .build();
        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    };

    builder.exec(no_wasm_transfer_request_1).commit();

    let balance_after = builder.get_purse_balance(default_account.main_purse());

    assert_eq!(balance_after, balance_before,);

    let last_result = builder.get_exec_result(0).unwrap().clone();
    let last_result = &last_result[0];

    assert!(
        last_result.as_error().is_some(),
        "Expected error but last result is {:?}",
        last_result
    );
    assert!(
        last_result.transfers().is_empty(),
        "Expected empty list of transfers"
    );
}
