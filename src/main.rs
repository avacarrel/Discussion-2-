/// Your crew's name. Both of you are going to change this line.
const CREW_NAME: &str = "the unnamed crew";

/// Your crew's motto. You will both change this one too, earlier and separately.
<<<<<<< HEAD
const MOTTO: &str = "we have not agreed on a motto";
=======
const MOTTO: &str = "the world is my oyster";
>>>>>>> a6d22a6da85dad7e213d25d63fc5c4d839168c56

fn main() {
    println!("=== {} ===", CREW_NAME);
    println!();
    println!("Crew roster:");

    // ROSTER: replace the line below with one for yourself.
    println!("  (nobody has signed on yet)");

    println!();
    println!("Motto: {}", MOTTO);
    println!("Report any problems to whoever merged last.");
}
