pub mod client;

pub mod network;

mod outermost {
    pub fn middle_func() {}

    fn _middle_private() {}

    mod inside {
        pub fn _inner_func() {}

        fn _inner_private() {}
    }
}

pub fn try_me() {
    outermost::middle_func(); // ok, can access because this is immediate parent
                              // outermost::_middle_private(); // borked, is private
                              // outermost::inside::_inner_func(); // borked, inside is private
                              // outermost::inside::_inner_private(); // borked, inside is private
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn client_can_connect() {
        client::connect();
    }
}
