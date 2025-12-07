use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Не достаточно аргументов");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // env::var возвращает экземпляр типа Result содержащий:
        // - Ok cодержащий значение переменной среды, если переменная среды задана
        // - Err, если переменная среды не задана.
        //is_err проверяет Result на Ok или Err /
        // eсли CASE_INSENSITIVE задана и имеет любое значение, то is_err вернет false и программа выполнит поиск не­ чувствительный к регистру.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> означает, что функция будет возвращать тип, который реализует типаж Error
// слово dyn расшифровывается как "динамический"
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // fs::read_to_string, бeрет имя файла, открывает его и возвращает Result<String>
    // -? возвращает значение ошибки из текущей функции
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    // в случае успеха
    Ok(())
}

// жизнений цикл соединяет фрaгмент contents и ожидаемое значение, пото му что компилятор не знает который из аргументов соотнести
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_test(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    // не чуствительность к регистру
    #[test]
    fn case_insensitive_test(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}