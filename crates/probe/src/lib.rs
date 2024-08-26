use miette::{Context, IntoDiagnostic, Result};
use tikv_client::{Key, Transaction, TransactionClient};

pub struct Client {
  pub client: TransactionClient,
}

async fn rollback(mut txn: Transaction) -> Result<()> {
  txn
    .rollback()
    .await
    .into_diagnostic()
    .context("failed to rollback transaction")
}

async fn commit(mut txn: Transaction) -> Result<()> {
  txn
    .commit()
    .await
    .into_diagnostic()
    .context("failed to commit transaction")?;
  Ok(())
}

impl Client {
  /// Create a new client.
  pub async fn new(addresses: Vec<String>) -> Result<Self> {
    let client = TransactionClient::new(addresses)
      .await
      .into_diagnostic()
      .context("failed to create tikv client")?;

    Ok(Self { client })
  }

  /// Get all key-value pairs.
  pub async fn get_all(&self) -> Result<Vec<(values::Value, values::Value)>> {
    let mut txn = self
      .client
      .begin_optimistic()
      .await
      .into_diagnostic()
      .context("failed to start optimistic transaction")?;

    let results = match txn
      .scan(Key::EMPTY.., 1000)
      .await
      .into_diagnostic()
      .context("failed to scan keys")
    {
      Ok(pairs) => pairs.collect::<Vec<_>>(),
      Err(e) => {
        rollback(txn).await?;
        return Err(e);
      }
    };

    commit(txn).await?;

    Ok(
      results
        .into_iter()
        .map(|p| (Vec::<u8>::from(p.0).into(), p.1.into()))
        .collect(),
    )
  }
}
