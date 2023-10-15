#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn square<T>(num: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy + std::ops::Add,
{
    num * num
}

impl User {
    fn increment_signin_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.username = String::from(new_email);
    }

    fn change_username(user: &mut User, new_username: &str) {
        user.username = String::from(new_username)
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = new_username.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn tests_struct() {
        let mut user_1 = User {
            username: "user1".to_string(),
            email: "user".to_string(),
            sign_in_count: 0,
            active: true,
        };

        change_username(&mut user_1, "mrigu");

        user_1.increment_signin_count();
square(10.0);

        dbg!(user_1);
    }
}
