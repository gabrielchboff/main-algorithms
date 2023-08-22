fn main() {
    let doors: [bool; 100] = [false; 100];
    let steps: u16 = 1;
    while steps <= 100 {
        for door in doors.iter() {
            *door = !*door;
        }
        steps += 1;
    }

    println!("The doors are {:?}", doors);
}
