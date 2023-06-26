// TODO:

use futures::future;

use libra_types::legacy_types::app_cfg::AppCfg;
use zapatos_sdk::rest_client::Client;
use zapatos_sdk::types::chain_id::ChainId;

pub async fn find_good_upstream(app_cfg: AppCfg) -> anyhow::Result<(Client, ChainId)> {
    // check if we can connect to this client, or exit
    let nodes = app_cfg.profile.upstream_nodes;

    let metadata =  future::select_all(
      nodes.into_iter().find_map(|u| async {
          let client = Client::new(u);
          match client.get_index().await {
              Ok(index) => Some((client, index.inner().chain_id)),
              _ => None,
          }
      })
  ).await?;

  Ok(metadata)
}