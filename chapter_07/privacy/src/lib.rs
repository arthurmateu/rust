pub mod outermost {
    //added pub here
    pub fn middle_function() {} //added pub here

    pub fn middle_secret_function() {} //added pub here

    pub mod inside {
        //added pub here
        pub fn inner_function() {}
        pub fn secret_function() {} //added pub here
    }
}

use outermost::inside::secret_function;

pub fn try_me() {
    //added pub here
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    secret_function();
}
