type Grid = Vec<Vec<bool>>;

/// Instantiates a grid of the given dimension.
pub fn instantiate_grid(dimension: u8, execute: bool) {
    let mut cmd = "cargo contract instantiate --suri //Alice --execute --args".to_string();
    let grid: Grid = vec![vec![false; dimension as usize]; dimension as usize];
    let grid = format!("{:?}", grid);
    let grid = grid.replace(" ", "");
    cmd = format!("{} {}", cmd, grid);
    println!("{cmd}");
    if execute {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
