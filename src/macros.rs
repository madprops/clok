#[macro_export]
macro_rules! p 
{
    ($left:expr, $right:expr) => 
    {
        println!($left, $right);
    };

    ($left:expr) => 
    {
        println!("{}", $left);
    };
}

#[macro_export]
macro_rules! pp 
{
    ($left:expr, $right:expr) => 
    {
        print!($left, $right);
    };

    ($left:expr) => 
    {
        print!("{}", $left);
    };
}

#[macro_export]
macro_rules! s 
{
    ($s: expr) => 
    {
        $s.to_string()
    };

    () => 
    {
        String::new()
    };
}