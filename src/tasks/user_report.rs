use loco_rs::prelude::*;
use task::Vars;

use crate::models::users;

pub struct UserReport;
#[async_trait]
impl Task for UserReport {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "user_report".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    // variables through the CLI:
    // `$ cargo loco task name:foobar count:2`
    // will appear as {"name":"foobar", "count":2} in `vars`
    async fn run(&self, app_context: &AppContext, vars: &Vars) -> Result<()> {
        let users = users::Entity::find().all(&app_context.db).await?;
        println!("args: {vars:?}");
        println!("!!! user_report: listing users !!!");
        println!("------------------------");
        for user in &users {
            println!("user: {}", user.email);
        }
        println!("done: {} users", users.len());
        Ok(())
    }
}
