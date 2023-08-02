// from here
// https://stackoverflow.com/questions/73008377/rust-create-test-setup

// rus test on cli
// cargo test --examples 
pub struct User {
    name: String,
    age: u8,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u8, weight: f32) -> Self {
        User {
            name: name,
            age: age,
            weight: weight,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }

    pub fn set_age(&mut self, new_age: u8) {
        self.age = new_age
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        user: User,
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown ...");
        }
    }

    fn setup() -> TestContext {
        println!("Test setup ...");

        TestContext {
            user: User::new(String::from("Bob"), 20, 150.23),
        }
    }

    #[test]
    fn test_user_get_name() {
        let ctx: TestContext = setup();
        assert_eq!(ctx.user.get_name(), "Bob")
    }

    #[test]
    fn test_user_set_name() {
        let mut ctx: TestContext = setup();
        ctx.user.set_name("Jane".to_string()); // no performance cost compared to String::from
        assert_eq!(ctx.user.get_name(), "Jane")
    }

    #[test]
    fn test_user_get_age() {
        let ctx: TestContext = setup();
        assert_eq!(ctx.user.get_age(), 20)
    }

    #[test]
    fn test_user_set_age() {
        let mut ctx = setup();
        ctx.user.set_age(25);
        assert_eq!(ctx.user.get_age(), 25)
    }

    #[test]
    fn test_user_get_weight() {
        let ctx = setup();
        assert_eq!(ctx.user.get_weight(), 150.23)
    }

    #[test]
    fn test_user_set_weight() {
        let mut ctx = setup();
        ctx.user.set_weight(160.8);
        assert_eq!(ctx.user.get_weight(), 160.8)
    }
}
