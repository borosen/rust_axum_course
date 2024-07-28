use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::base::{self, DbBmc};

// region:    --- Task Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize, Fields)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize, Fields, Default)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskFilter {
    id: Option<OpValsInt64>,
    title: Option<OpValsString>,
    done: Option<OpValsBool>,
}

// endregion: --- Task Types

// region:    --- TaskBmc
pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, data: TaskForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, data).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<TaskFilter>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Task>> {
        base::list::<Self, _, _>(ctx, mm, filter, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, data: TaskForUpdate) -> Result<()> {
        base::update::<Self, TaskForUpdate>(ctx, mm, id, data).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// endregion: --- TaskBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;

        assert_eq!(task.title, fx_title);

        TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                }),
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_all_ok-task 01", "test_list_all_ok-task 02"];

        let fx_tasks = _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        let tasks = TaskBmc::list(&ctx, &mm, None, None).await?;

        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_all_ok-task"))
            .collect();

        assert_eq!(tasks.len(), 2, "number of seeded tasks.");

        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_filter_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &[
            "test_list_by_filter_ok-task 01.a",
            "test_list_by_filter_ok-task 01.b",
            "test_list_by_filter_ok-task 02.a",
            "test_list_by_filter_ok-task 02.b",
            "test_list_by_filter_ok-task 03",
        ];

        let fx_tasks = _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let filter = serde_json::from_value(json!({
            "title": {"$endsWith": ".a",
            "$containsAny": ["01", "02"]},
        }))
        .ok();

        let list_options = serde_json::from_value(json!({
            "limit": 1,
            "order_bys": "!id",
        }))
        .ok();
        let tasks = TaskBmc::list(&ctx, &mm, filter, list_options).await?;

        // -- Check
        println!("->> {tasks:#?}");

        // -- Cleanup
        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok task 01";
        let fx_title_new = "test_create_ok task 01 - updated";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        let task_u = TaskForUpdate {
            title: Some(fx_title_new.to_owned()),
            ..Default::default()
        };

        TaskBmc::update(&ctx, &mm, fx_task.id, task_u).await?;

        let updated_task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;

        assert_eq!(updated_task.title, fx_title_new, "Title should be updated");

        TaskBmc::delete(&ctx, &mm, fx_task.id).await;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                }),
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}

// endregion: --- Tests
