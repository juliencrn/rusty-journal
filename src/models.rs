use super::schema;
use super::schema::tasks;
use diesel::{sqlite::SqliteConnection, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::fmt;

#[derive(Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub text: String,
    pub completed: i32,
    // pub created_at: chrono::NaiveDateTime,
    // pub created_at: String,
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub text: String,
    pub completed: i32,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkbox = if self.completed == 1 { "[*]" } else { "[ ]" };
        write!(f, "{} - [{}] {:<50}", checkbox, self.id, self.text)
    }
}

pub fn add(text: String, connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::tasks;

    let new_task = NewTask { text, completed: 0 };

    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(connection)
        .expect(&format!("Unable to create task"));

    println!("Task created");

    Ok(())
}

pub fn list(connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::{completed, tasks};

    let results = tasks
        .filter(completed.eq(0))
        .limit(10)
        .load::<Task>(connection)
        .expect(&format!("Unable to list tasks"));

    println!("Todo List (uncompleted tasks only):");
    println!("-------------------");

    for task in results {
        println!("{}", task);
    }

    Ok(())
}

pub fn list_all(connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::tasks;

    let results = tasks
        .load::<Task>(connection)
        .expect(&format!("Unable to list tasks"));

    println!("Todo List:");
    println!("-------------------");

    for task in results {
        println!("{}", task);
    }

    Ok(())
}

pub fn complete(id: i32, connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::{completed, tasks};

    diesel::update(tasks.find(id))
        .set(completed.eq(1))
        .execute(connection)
        .expect(&format!("Unable to complete task {}", id));

    println!("Task {} completed", id);

    Ok(())
}

pub fn update(id: i32, new_text: String, connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::{tasks, text};

    diesel::update(tasks.find(id))
        .set(text.eq(new_text))
        .execute(connection)
        .expect(&format!("Unable to update task {}", id));

    println!("Task {} updated", id);

    Ok(())
}

pub fn delete(id: i32, connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks::dsl::tasks;

    diesel::delete(tasks.find(id))
        .execute(connection)
        .expect(&format!("Unable to delete task {}", id));

    println!("Task {} deleted", id);

    Ok(())
}