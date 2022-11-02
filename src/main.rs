use crossterm::{self, execute, terminal, Result, cursor, style};

struct Point<T> {
    x: T,
    y: T,
}

fn draw_line_by_x (stdout: &mut std::io::Stdout, a: &Point<u16>, b: &Point<u16>) -> Result<()> {
    // let Point { mut x, mut y } = a;
    

    let from = if a.x > b.x { b } else { a };
    let to = if a.x > b.x { a } else { b };

    let growth = (to.y as f64 - from.y as f64) / (to.x as f64 - from.x as f64);

    let mut x = from.x;
    // let mut y = from.y;

    while x < to.x {
        let position = growth * (x - from.x) as f64 + from.y as f64;
        execute!(
            stdout,
            cursor::MoveTo(x, position as u16),
            style::SetForegroundColor(style::Color::White),
            style::Print("0")
        )?;

        x += 1;
    }

    Ok(())
}

fn draw_line_by_y (stdout: &mut std::io::Stdout, a: &Point<u16>, b: &Point<u16>) -> Result<()> {
    // let Point { mut x, mut y } = a;
    

    let from = if a.y > b.y { b } else { a };
    let to = if a.y > b.y { a } else { b };

    let growth = (to.x as f64 - from.x as f64) / (to.y as f64 - from.y as f64);

    let mut y = from.y;
    // let mut y = from.y;

    while y < to.y {
        let position = growth * (y - from.y) as f64 + from.x as f64;
        execute!(
            stdout,
            cursor::MoveTo(position as u16, y),
            style::SetForegroundColor(style::Color::White),
            style::Print("0")
        )?;

        y += 1;
    }

    Ok(())
}

fn draw_line (stdout: &mut std::io::Stdout, a: &Point<u16>, b: &Point<u16>) -> Result<()> {
    if (a.x as f64 - b.x as f64).abs() < (a.y as f64 - b.y as f64).abs() {
        draw_line_by_y(stdout, b, a)
    } else {
        draw_line_by_x(stdout, a, b)
    }
}

mod matrix;

use crate::matrix::Matrix;

//
//
//

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn point_to_matrix (p: &Point3D) -> Matrix {
    Matrix {
        rows: 1,
        cols: 4,
        data: vec![p.x, p.y, p.z, 1.0],
    }
}


// execute!(
//     stdout,
//     cursor::MoveTo((size.0 / 2) as u16 - (&command.len() / 2) as u16, (size.1 / 2) as u16),
//     style::SetForegroundColor(style::Color::White),
//     style::Print(&command)
// )?;

fn main() -> Result<()>{
    let mut stdout = std::io::stdout();
    let size = crossterm::terminal::size()?;

    crossterm::terminal::enable_raw_mode()?;

    execute!(stdout, terminal::EnterAlternateScreen)?;

    // Do anything on the alternate screen




    let cube = [
        Point3D { x: -1.0, y: -1.0, z: -1.0 },
        Point3D { x:  1.0, y: -1.0, z: -1.0 },
        Point3D { x: -1.0, y:  1.0, z: -1.0 },
        Point3D { x:  1.0, y:  1.0, z: -1.0 },
        Point3D { x: -1.0, y: -1.0, z:  1.0 },
        Point3D { x:  1.0, y: -1.0, z:  1.0 },
        Point3D { x: -1.0, y:  1.0, z:  1.0 },
        Point3D { x:  1.0, y:  1.0, z:  1.0 },
    ];

    let lines = [
        // z stuff
        (0, 1), (1, 3), (3, 2), (2, 0),
        (4, 5), (5, 7), (7, 6), (6, 4),

        // y stuff
        (0, 1), (1, 5), (5, 4), (4, 0),
        (2, 3), (3, 7), (7, 6), (6, 2),

        // x stuff
        (0, 2), (2, 6), (6, 4), (4, 0),
        (1, 3), (3, 7), (7, 5), (5, 1),

        // (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
        // (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
        // (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
        // (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6), (3, 7),
        // (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7),
        // (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6), (5, 7),
        // (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7),
        // (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7),
    ];

    let translate = matrix![
        1.0, 0.0, 0.0, 0.0;
        0.0, 1.0, 0.0, 0.0;
        0.0, 0.0, 1.0, 0.0;
        4.0, 2.0, 0.0, 1.0;
    ];

    let scale = matrix![
        15.0,  0.0,  0.0, 0.0;
         0.0,  7.5,  0.0, 0.0;
         0.0,  0.0, 15.0, 0.0;
         0.0,  0.0,  0.0, 1.0;
    ];
    
    // let angle: f64 = 3.14 / 2.0;
    let angle: f64 = 0.5;

    let pose = matrix![
        angle.cos(),  -angle.sin(), 0.0, 0.0;
        angle.sin(),  angle.cos(),  0.0, 0.0;
        0.0,  0.0,  1.0, 0.0;
        0.0,  0.0,  0.0, 1.0;
    ];

    let mut angle = 0f64;

    for i in 0..10000 {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
        )?;

        let rotate = matrix![
            angle.cos(),  0.0,  angle.sin(), 0.0;
            0.0,  1.0,  0.0, 0.0;
            -angle.sin(),  0.0,  angle.cos(), 0.0;
            0.0,  0.0,  0.0, 1.0;
        ];

        // let rotate = matrix![
        //     angle.cos(),  -angle.sin(), 0.0, 0.0;
        //     angle.sin(),  angle.cos(),  0.0, 0.0;
        //     0.0,  0.0,  1.0, 0.0;
        //     0.0,  0.0,  0.0, 1.0;
        // ];

        angle += 0.005;
        
        for line in lines {
            let point_a = &cube[line.0];
            let point_b = &cube[line.1];

            let point_a_matrix = point_to_matrix(point_a);
            let point_b_matrix = point_to_matrix(point_b);

            let mut a_matrix = point_a_matrix * pose.clone()* rotate.clone() * translate.clone() * scale.clone();
            let mut b_matrix = point_b_matrix * pose.clone()* rotate.clone() * translate.clone() * scale.clone();

            // a_matrix.data[3] = 1.0;
            // b_matrix.data[3] = 1.0;

            let from = Point {
                x: (a_matrix.data[0] / a_matrix.data[3]) as u16,
                y: (a_matrix.data[1] / a_matrix.data[3]) as u16,
            };

            let to = Point {
                x: (b_matrix.data[0] / b_matrix.data[3]) as u16,
                y: (b_matrix.data[1] / b_matrix.data[3]) as u16,
            };

            // Multiply and do stuff

            draw_line(&mut stdout, &from, &to)?;
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
    }


    // let cube = [
    //     Point { x: 10, y: 10 },
    //     Point { x: 25, y: 10 },
    //     Point { x: 15, y: 15 },
    //     Point { x: 10, y: 15 },
    // ];

    // let mut i = 0;
    // while i < 1000 {
    //     draw_line(&mut stdout, &cube[0], &cube[1])?;
    //     draw_line(&mut stdout, &cube[1], &cube[2])?;
    //     draw_line(&mut stdout, &cube[2], &cube[3])?;
    //     draw_line(&mut stdout, &cube[3], &cube[0])?;
    //     // let mut prev_point = &cube[0];

    //     // for point in cube.iter() {
    //     //     draw_line(&mut stdout, point, prev_point)?;

    //     //     prev_point = point;
    //     // }

    //     // draw_line(&mut stdout, prev_point, &cube[0])?;

    //     i+=1;
    // }

    execute!(stdout, terminal::LeaveAlternateScreen)?;

    Ok(())
}
