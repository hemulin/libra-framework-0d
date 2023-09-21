use anyhow::Context;
use diem_forge::DiemPublicInfo;
use diem_sdk::rest_client::Client;
use diem_types::account_address::AccountAddress;
use libra_cached_packages::libra_stdlib;
use libra_types::{type_extensions::client_ext::ClientExt, move_resource::gas_coin::SlowWalletBalance};

/// Get the balance of the 0L coin
pub async fn get_libra_balance(
    client: &Client,
    address: AccountAddress,
) -> anyhow::Result<SlowWalletBalance> {
    let res = client
        .view_ext("0x1::slow_wallet::balance", None, Some(address.to_string()))
        .await?;
    dbg!(&res);

    let move_tuple = serde_json::from_value::<Vec<String>>(res)?;

    let b = SlowWalletBalance {
      unlocked: move_tuple[0].parse().context("no value found")?,
      total: move_tuple[1].parse().context("no value found")?,
    };

    Ok(b)
}

pub async fn mint_libra(
    public_info: &mut DiemPublicInfo<'_>,
    addr: AccountAddress,
    amount: u64,
) -> anyhow::Result<()> {
    let payload = public_info
        .transaction_factory()
        .payload(libra_stdlib::gas_coin_mint_to_impl(addr, amount));

    let mint_txn = public_info
        .root_account()
        .sign_with_transaction_builder(payload);

    public_info.client().submit_and_wait(&mint_txn).await?;
    Ok(())
}
