// use super::models;
// use super::rusty_journal;
use super::models::{NewTask, Task};
use super::schema;
// use super::tasks;
// use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use diesel::prelude::*;
// use diesel::serialize::Result;
// use serde::Deserialize;
// use serde::Serialize;
use std::fmt;
// use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom, Write};
// use std::path::PathBuf;
use diesel::{
    result::Error as DieselError, sql_types::BigInt, sqlite::SqliteConnection, Connection,
    ExpressionMethods, QueryDsl, RunQueryDsl,
};

// impl NewTask<'_> {
//     pub fn new(text: &str) -> NewTask {
//         let created_at: DateTime<Utc> = Utc::now();
//         NewTask {
//             text,
//             created_at,
//             completed: 0,
//         }
//     }
// }

pub fn add(text: String, connection: &SqliteConnection) -> anyhow::Result<()> {
    use schema::tasks;
    use schema::tasks::dsl::id;
    no_arg_sql_function!(last_insert_rowid, i64);
    no_arg_sql_function!(
        now,
        diesel::sql_types::Timestamp,
        "Represents the SQL NOW() function"
    );

    let new_task = NewTask { text, completed: 0 };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(connection)?;

    // let new_task_id = diesel::select(last_insert_rowid)
    //     .limit(1)
    //     .load::<Task>(connection)?;

    // let last_added_task = tasks::table
    //     .filter(id.eq(new_task_id))
    //     .first::<Task>(connection)?;

    Ok(())

    // let results = tasks::table
    //     .filter(tasks::dsl::text.like(format!("{}", new_task.text)))
    //     .first::<Task>(connection)?;

    // return results;
}

pub fn list(connection: &SqliteConnection, mut writer: impl Write) -> anyhow::Result<()> {
    use schema::tasks;
    use schema::tasks::dsl::*;

    let results = tasks::table
        .filter(completed.eq(0))
        .limit(10)
        .load::<Task>(connection)?;

    for task in results {
        writeln!(writer, "{}", task)?;
    }

    Ok(())
}

pub fn update(id: i32, text: String, connection: &SqliteConnection) -> anyhow::Result<()> {
    unimplemented!()
}

pub fn delete(id: i32, connection: &SqliteConnection) -> anyhow::Result<()> {
    unimplemented!()
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // let checkbox = if self.completed == 1 { "[*]" } else { "[]" };
        write!(f, "[{}] {:<50}", self.id, self.text)
    }
}

// pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
//     let file = open_journal_file(journal_path)?;
//     let mut tasks = collect_tasks(&file)?;

//     // Write the modified task list back into the file.
//     tasks.push(task);
//     serde_json::to_writer(file, &tasks)?;

//     Ok(())
// }

// pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
//     let file = open_journal_file(journal_path)?;
//     let mut tasks = collect_tasks(&file)?;

//     // Remove the task.
//     if task_position == 0 || task_position > tasks.len() {
//         return Err(Error::new(ErrorKind::InvalidInput, "Invalid task position"));
//     }
//     tasks.remove(task_position - 1);

//     // Write the modified task list back into the file.
//     file.set_len(0)?;
//     serde_json::to_writer(file, &tasks)?;

//     Ok(())
// }

// pub fn list_tasks(journal_path: PathBuf, mut writer: impl Write) -> Result<()> {
//     let file = open_journal_file(journal_path)?;
//     let tasks = collect_tasks(&file)?;

//     // Enumerate and display tasks, if any.
//     if tasks.is_empty() {
//         writeln!(writer, "Task list is empty!")?;
//     } else {
//         let mut order: u32 = 1;

//         for task in tasks {
//             writeln!(writer, "{}: {}", order, task)?;
//             order += 1;
//         }
//     }

//     Ok(())
// }

///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////

// This function opens a file, seek the cursor to the first position and returns a list of tasks.
// fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
//     // Rewind the file before reading it.
//     file.seek(SeekFrom::Start(0))?;

//     // Consume the file's contents as a vector of tasks.
//     let tasks: Vec<Task> = match serde_json::from_reader(file) {
//         Ok(tasks) => tasks,
//         Err(e) if e.is_eof() => Vec::new(),
//         Err(e) => Err(e)?,
//     };

//     // Rewind the file after reading it.
//     file.seek(SeekFrom::Start(0))?;

//     Ok(tasks)
// }

// fn open_journal_file(journal_path: PathBuf) -> Result<File> {
//     let file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(journal_path)?;
//     Ok(file)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::str;
//     use tempfile::{Builder, NamedTempFile};

//     fn get_tasks(tmp_file: &NamedTempFile) -> Result<Vec<Task>> {
//         let file = tmp_file.reopen()?;
//         let tasks = collect_tasks(&file).unwrap();
//         Ok(tasks)
//     }

//     #[test]
//     fn test_add_task() -> Result<()> {
//         // Create a tmp file and a task
//         let tmp_file = Builder::new().tempfile()?;
//         let test_text = String::from("Hello, world!");

//         // [test]
//         let path = PathBuf::from(tmp_file.path());
//         let task = Task::new(test_text.clone());
//         add_task(path, task)?;

//         // Get the created task reading the tmp file
//         let tasks = get_tasks(&tmp_file)?;
//         let result = tasks.get(0).unwrap();

//         assert_eq!(result.text, test_text);

//         drop(tmp_file);
//         Ok(())
//     }

//     #[test]
//     fn test_complete_task() -> Result<()> {
//         let tmp_file = Builder::new().tempfile()?;

//         // Create test task and checks if it's created in the tmp file
//         let path = PathBuf::from(tmp_file.path());
//         let task = Task::new(String::from("Hello, world!"));

//         add_task(path, task)?;

//         let tasks = get_tasks(&tmp_file)?;

//         assert_eq!(tasks.len(), 1);

//         // [test] Now remove it an test if we have a empty array from the tmp file
//         let path = PathBuf::from(tmp_file.path());

//         complete_task(path, 1)?;

//         // Get the result from the tmp file
//         let tasks = get_tasks(&tmp_file)?;

//         assert_eq!(tasks.len(), 0);

//         drop(tmp_file);
//         Ok(())
//     }

//     #[test]
//     fn test_empty_list_tasks() -> Result<()> {
//         let tmp_file = Builder::new().tempfile()?;
//         let path = PathBuf::from(tmp_file.path());
//         let mut result = Vec::new();

//         list_tasks(path, &mut result)?;

//         assert_eq!(result, b"Task list is empty!\n");
//         Ok(())
//     }

//     #[test]
//     fn test_list_tasks() -> Result<()> {
//         // Create test task and checks if it's created in the tmp file
//         let tmp_file = Builder::new().tempfile()?;
//         let path = PathBuf::from(tmp_file.path());
//         let task = Task::new(String::from("Hello, world!"));

//         add_task(path, task)?;

//         let tasks = get_tasks(&tmp_file)?;

//         assert_eq!(tasks.len(), 1);

//         // [test] Now compare stdout
//         let mut result = Vec::new();
//         let path = PathBuf::from(tmp_file.path());

//         list_tasks(path, &mut result)?;

//         let string_result = str::from_utf8(&result).unwrap();

//         assert!(string_result.contains("1: Hello, world!"));
//         Ok(())
//     }
// }
