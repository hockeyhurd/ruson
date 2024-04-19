use crate::log::logger::{EnumLogLevel, get_log_level_from_string, get_std_logger, ILogger};

pub struct CLIArgs
{
    pub input_file: Option<String>,
    pub log_level: EnumLogLevel,
}

impl CLIArgs
{
    pub fn new() -> Self
    {
        Self { input_file: None, log_level: EnumLogLevel::WARN }
    }

    pub fn parse(&mut self, args: &Vec<String>) -> Option<(i32, String)>
    {
        let arg_count = args.len();

        if arg_count <= 1
        {
            return Some((-1, String::from("Expected at least one argument")));
        }

        let mut skip_next = false;

        for i in 1..arg_count
        {
            if skip_next
            {
                skip_next = false;
                continue;
            }

            let arg = args.get(i).expect("Failed to unwrap argument");

            if arg == "--input"
            {
                let opt_next_arg = args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-1, String::from("Expected an input file after the argument '--input'")));
                }

                self.input_file = Some(opt_next_arg.unwrap().clone());
                skip_next = true;
            }

            else if arg == "--log-level"
            {
                let opt_next_arg = args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-1, String::from("Expected an input file after the argument '--log-level'")));
                }

                let result_log_level = get_log_level_from_string(opt_next_arg.unwrap());
                let logger_cell = get_std_logger().lock().unwrap();
                let mut logger = logger_cell.borrow_mut();

                match result_log_level
                {
                    Ok(log_level) =>
                    {
                        self.log_level = log_level;
                        logger.set_log_level(log_level);
                    },
                    Err(msg) =>
                    {
                        logger.warn(msg);
                    },
                }

                skip_next = true;
            }

            else
            {
                let mut err_msg = String::from("Invalid argument ('");
                err_msg += &arg;
                err_msg += &String::from("')");

                return Some((-2, err_msg));
            }
        }

        // return Some((-1, String::from("Failed to parse input arguments.")));
        return None;
    }
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use crate::utils::cli_args::CLIArgs;
    use crate::log::logger::EnumLogLevel;

    fn cli_assert_fail(opt_err_pair: Option<(i32, String)>)
    {
        assert!(opt_err_pair.is_some());

        let (err_code, err_msg) = opt_err_pair.unwrap();
        assert_ne!(err_code, 0);
        assert!(!err_msg.is_empty());
    }

    #[test]
    fn parse_empty_input_fail()
    {
        let args: Vec<String> = vec![ String::from("rson"); 1];
        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        cli_assert_fail(opt_err_pair);
    }

    #[test]
    fn parse_one_input_invalid_arg_fail()
    {
        let mut args = Vec::<String>::with_capacity(2);
        args.push(String::from("rson"));
        args.push(String::from("--args"));

        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        cli_assert_fail(opt_err_pair);
    }

    #[test]
    fn parse_one_input_missing_value_fail()
    {
        let mut args = Vec::<String>::with_capacity(2);
        args.push(String::from("rson"));
        args.push(String::from("--input"));

        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        cli_assert_fail(opt_err_pair);
    }

    #[test]
    fn parse_two_input_expect_input_arg_value_valid()
    {
        let file = String::from("myfile.json");
        let mut args = Vec::<String>::with_capacity(3);
        args.push(String::from("rson"));
        args.push(String::from("--input"));
        args.push(file.clone());

        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        assert!(opt_err_pair.is_none());
        assert!(cli_args.input_file.is_some());
        assert_eq!(&cli_args.input_file.unwrap(), &file);
    }

    #[test]
    fn parse_two_input_expect_input_arg_value_valid2()
    {
        let mut args = Vec::<String>::with_capacity(3);
        args.push(String::from("rson"));
        args.push(String::from("--log-level"));
        args.push(String::from("DEBUG"));

        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        assert!(opt_err_pair.is_none());
        assert_eq!(cli_args.log_level, EnumLogLevel::DEBUG);
    }

    #[test]
    fn parse_all_input_expect_valid()
    {
        let file = String::from("myfile.json");
        let mut args = Vec::<String>::with_capacity(5);
        args.push(String::from("rson"));
        args.push(String::from("--log-level"));
        args.push(String::from("DEBUG"));
        args.push(String::from("--input"));
        args.push(file.clone());

        let mut cli_args = CLIArgs::new();

        let opt_err_pair = cli_args.parse(&args);
        assert!(opt_err_pair.is_none());
        assert!(opt_err_pair.is_none());
        assert_eq!(cli_args.log_level, EnumLogLevel::DEBUG);
        assert!(cli_args.input_file.is_some());
        assert_eq!(&cli_args.input_file.unwrap(), &file);
    }
}

