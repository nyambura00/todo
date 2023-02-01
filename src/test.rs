#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_edit_with_args() {
        // create a dummy todo file to test against
        let mut todo_file = File::create("todos.txt").unwrap();
        todo_file.write_all(b"todo1\ntodo2\ntodo3").unwrap();

        let todo_app = Todo {
            todo_path: "todos.txt".to_string(),
        };

        let args = vec!["1".to_string(), "new_todo".to_string()];

        todo_app.edit(&args);

        // check if the file was edited correctly
        let mut file = File::open("todos.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "todo2\ntodo3\nnew_todo");
    }

    #[test]
    fn test_edit_without_args() {
        let todo_app = Todo {
            todo_path: "todos.txt".to_string(),
        };

        let args = vec![];

        let result = std::panic::catch_unwind(|| todo_app.edit(&args));
        assert!(result.is_err());
    }
}