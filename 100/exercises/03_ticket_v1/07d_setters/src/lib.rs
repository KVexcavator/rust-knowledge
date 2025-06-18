// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

fn check_not_empty(value: &str, field_name: &str) {
    if value.is_empty() {
        panic!("{field_name} cannot be empty");
    }
}
fn check_len(value: &str, field_name: &str, len: usize){
    if value.len() > len {
        panic!("{field_name} cannot be longer than {len} bytes")
    }
}
fn check_status(value: &str) {
    if value != "To-Do" && value != "In Progress" && value != "Done" {
        panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
    }
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        check_not_empty(&title, "Title");
        check_len(&title, "Title", 50);
        check_not_empty(&description, "Description");
        check_len(&description, "Description" , 500);
        check_status(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, new_title: String) {
        check_not_empty(&new_title, "Title");
        check_len(&new_title, "Title", 50);
        self.title = new_title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, new_description: String){
        check_not_empty(&new_description, "Description");
        check_len(&new_description, "Description" , 500);
        self.description = new_description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, new_status: String) {
        check_status(&new_status);
        self.status = new_status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
