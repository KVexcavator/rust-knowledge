use std::collections::HashMap;

// Каждая строка на входе может быть одного из следующих типов:
#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

//Если ContentType — TemplateVariable, содержимое строки анализируется и сохраняется в ExpressionData struct.alloc
//Если файл шаблона содержит эту строку: <p> Hello {{name}} ,welcome </p> then
// Head = "Hello", variable = "name" and tail = ",welcome"
#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

//это проверяет, содержится ли строка символов в другой строке
pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

// Это проверяет наличие соответствующих строк символов в заданных входных данных
pub fn check_matching_pair(input: &str, symbol1: &str, symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

// возвращает индекс заданного символа char, если символ присутствует
fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;
    while let Some((c,d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist, index)
}

// Вспомогательная функция для анализа переменной шаблона
fn get_expression_data(input_line: &str) -> ExpressionData {
    let (_h,i) = get_index_for_symbol(input_line,'{');
    let head = input_line[0..i].to_string();
    let (_j,k) = get_index_for_symbol(input_line,'}');
    let variable = input_line[i+1+1..k].to_string();
    let tail = input_line[k+1+1..].to_string();

    ExpressionData{
        head: Some(head),
        variable: variable,
        tail: Some(tail),
    }
}

// Считывает одну строку файла шаблона и возвращает тип Content. Типы контента определены в перечислении ContentType.
// Если contentType — TemplateVariable, строка анализируется дальше, чтобы выделить заголовок, переменную и конец.
// Если ContentType — Literal, возвращается прочитанный входной файл без каких-либо изменений
pub fn get_content_type(input_line: &str) -> ContentType {
    // Выражения тегов заключены в {% и %}
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");

    // Выражения ForTag начинаются с ключевых слов «for» и «in», заключённых в фигурные скобки {% и %}
    // Выражения ForTag заканчиваются ключевым словом «endfor», заключённым в фигурные скобки {% и %}
    let is_for_tag = (check_symbol_string(&input_line, "for") && check_symbol_string(&input_line, "in")) || check_symbol_string(&input_line, "endfor");

    // Выражения IfTag начинаются с ключевого слова «if», заключённого в {% и %}
    // Выражения IfTag заканчиваются ключевым словом «endif», заключённым в {% и %}
    let is_if_tag = check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");

    // Шаблонные переменные имеют
    // 1) необязательную заглавную часть,
    // 2) переменную шаблона, заключенную в фигурные скобки {{ и }}
    // 3) необязательную заглавную часть
    // Например, выражение <p> Hello {{name}} ,welcome </p> анализируется следующим образом:
    // head = 'Hello', variable = 'name' и tail = ',welcome'
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    let return_val;
    // случай For Tag
    if is_tag_expression && is_for_tag{
        return_val = ContentType::Tag(TagType::ForTag);
    // случай If Tag
    } else if is_tag_expression && is_if_tag{
        return_val = ContentType::Tag(TagType::IfTag);
    // случай с шаблонной переменной
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    // случай с Literal
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    // неизвестный тип
    } else {
        return_val = ContentType::Unrecognized;
    }
    return_val
}

// Функция для генерации HTML для строки, содержащей переменную шаблона
pub fn generate_html_template_var(
    content: ExpressionData,
    context: HashMap<String, String>,
) -> String {
    let mut html = String::new();
    println!("expression data is {:?}", content);

    if let Some(h) = content.head{
        html.push_str(&h);
    }

    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val);
    }

    if let Some(t) = content.tail {
        html.push_str(&t);
    }

    html
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test(){
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}} ,welcome")
        );
    }

    #[test]
    fn check_for_tag_test(){
        assert_eq!{
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %} ,welcome")
        };
    }

    #[test]
    fn check_if_tag_test(){
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}")
        );
    }

    #[test]
    fn check_symbol_string_test(){
        assert_eq!(
            true, 
            check_symbol_string("{{Hello}}","{{")
        );
    }

    #[test]
    fn check_symbol_pair_test() {
        assert_eq!(
            true,
            check_matching_pair("{{Hello}}","{{","}}")
        );
    }

    #[test]
    fn check_get_index_for_symbol_test(){
        assert_eq!(
            (true, 3),
            get_index_for_symbol("Hi {name} ,welcome", '{')
        );
    }

    #[test]
    fn check_get_expression_data_test(){
        let expression_data = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string())
        };
        assert_eq!(
            expression_data,
            get_expression_data("Hi {{name}} ,welcome")
        );
    }
}
