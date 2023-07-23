pub struct AppError {
    message: String,
    code: String,
}

impl AppError {
    pub fn default() -> Self {
        // Empty error
        Self {
            message: String::new(),
            code: String::new(),
        }
    }

    pub fn generation_too_big() -> Self {
        // Error in case the number asked for is too big
        Self {
            message: String::from("Generation too big"),
            code: String::from("GEN_BIG"),
        }
    }
    pub fn missing_db() -> Self {
        // Error in case SQLite not working
        Self {
            message: String::from("DB not found, or unreachable"),
            code: String::from("DB_PROBLEM"),
        }
    }
    pub fn missing_cmd_env() -> Self {
        // In case the script does not have any params
        Self {
            message: String::from("Missing params on the app"),
            code: String::from("NO_PARAMS"),
        }
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} - {}",
            String::from(self.code.clone()),
            String::from(self.message.clone())
        );
    }
}
