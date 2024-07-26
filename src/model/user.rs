use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::{
    base::{self, DbBmc},
    Error, ModelManager, Result,
};
use modql::field::{Fields, HasFields};
use sea_query::{query, Expr, Mode, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use super::base::CommonIden;

// region:    --- User Types

#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
struct UserForInsert {
    username: String,
}

#[derive(Clone, Debug, Fields, FromRow)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // -- pwd and token info
    pub pwd: Option<String>, // hashed, #_sheme_id_#...
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, Debug, Fields, FromRow)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

/// Market trait
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForAuth {}
impl UserBy for UserForLogin {}

// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let (sql, values) = Query::select()
            .from(Self::table_ref())
            .and_where(Expr::col(CommonIden::Username).eq(username))
            .columns(E::field_column_refs())
            .limit(1)
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn update_pdw(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;

        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        let (sql, values) = Query::update()
            .table(Self::table_ref())
            .values([(CommonIden::Pwd, pwd.into())])
            .and_where(Expr::col(CommonIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::{Context, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_first_by_username_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "demo1";

        let user = UserBmc::first_by_username::<User>(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
