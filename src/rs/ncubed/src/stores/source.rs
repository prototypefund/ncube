use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Source, Workspace};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;
use tracing::instrument;

use crate::db::{http, sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn source_store(wrapped_db: Database) -> Box<dyn SourceStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(SourceStoreSqlite { db }),
        Database::Http(client) => Box::new(SourceStoreHttp { client }),
    }
}

#[async_trait]
pub(crate) trait SourceStore {
    async fn exists(&self, id: i32) -> Result<bool, StoreError>;
    async fn create(&self, kind: &str, term: &str) -> Result<(), StoreError>;
    async fn list(&self, workspace: &Workspace) -> Result<Vec<Source>, StoreError>;
    async fn delete(&self, id: i32) -> Result<(), StoreError>;
    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct SourceStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl SourceStore for SourceStoreSqlite {
    #[instrument]
    async fn exists(&self, id: i32) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&id], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
    async fn create(&self, kind: &str, term: &str) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/create.sql"))?;

        stmt.execute(params![&kind, &term, &now.to_rfc3339(), &now.to_rfc3339()])?;

        Ok(())
    }

    #[instrument]
    async fn list(&self, _workspace: &Workspace) -> Result<Vec<Source>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/list.sql"))?;

        let mut sources: Vec<Source> = vec![];
        for source in from_rows::<Source>(stmt.query(NO_PARAMS)?) {
            sources.push(source?);
        }

        Ok(sources)
    }

    #[instrument]
    async fn delete(&self, id: i32) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/delete.sql"))?;

        stmt.execute(params![&id])?;

        Ok(())
    }

    #[instrument]
    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/update.sql"))?;

        stmt.execute(params![&id, &kind, &term, &now.to_rfc3339()])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SourceStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl SourceStore for SourceStoreHttp {
    async fn exists(&self, _id: i32) -> Result<bool, StoreError> {
        todo!()
    }

    async fn create(&self, _kind: &str, _term: &str) -> Result<(), StoreError> {
        todo!()
    }

    #[instrument]
    async fn list(&self, workspace: &Workspace) -> Result<Vec<Source>, StoreError> {
        let url_path = format!("/api/workspaces/{}/sources", workspace.slug);
        let data: Vec<Source> = self.client.get(&url_path).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    async fn delete(&self, _id: i32) -> Result<(), StoreError> {
        todo!()
    }

    async fn update(&self, _id: i32, _kind: &str, _term: &str) -> Result<(), StoreError> {
        todo!()
    }
}
