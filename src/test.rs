use std::fs::OpenOptions;

#[cfg(test)]
mod tests {
    use crate::Todo;

    // for the edit function
    #[test]
    fn right_args_number(&[args]: &[str]) {
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn arg_one_type_usize(&[args]: [&str]) {
        let index_arg: usize = args[0].to_string().parse::<usize>().unwrap();
        assert!(typeof(index_arg), usize);
    }

    #[test]
    fn arg_two_type_string() {
        let formatted_todo: String = args[1].to_string().parse::<String>().unwrap();
        assert!(typeof(formatted_todo), String);
    }

    #[test]
    fn edit_fn_works(self, &[args]: [String]) {

        // the respective args
        let index_to_update = args[0].to_string().parse::<usize>().unwrap();
        let formatted_todo = args[1].to_string().parse::<String>().unwrap();

        
    }

    #[test]
    fn respective_index_updated(self, &[args]: [String]) {
        // the respective args
        let index_to_update = args[0].to_string().parse::<usize>().unwrap();
        let formatted_todo = args[1].to_string().parse::<String>().unwrap();
    }
}