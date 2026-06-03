// EX1:
// fn loop_then_return(mut counter: i32) -> i32 {
//     loop {
//         counter += 1;
//         if counter % 50 == 0 {
//             break;
//         }
//     }

//     counter
// }

// fn main() {
//     let my_number;

//     {
//         let number = { 57 };
//         my_number = loop_then_return(number);
//     }

//     println!("{}", my_number);
// }

//EX2: CLOSURE
// fn main() {
//     let x = 2;

//     let square = |i| -> i32 { i * i }(x);

//     println!("{}", square);
// }

//CODE: TRÒ RẮN SĂN MỒI
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Khởi tạo terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut snake = vec![Point { x: 10, y: 10 }];
    let mut dir = Direction::Right;
    let mut food = Point { x: 15, y: 10 };
    let mut last_tick = Instant::now();

    loop {
        // 1. Xử lý Input (không chặn vòng lặp)
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => dir = Direction::Up,
                    KeyCode::Down => dir = Direction::Down,
                    KeyCode::Left => dir = Direction::Left,
                    KeyCode::Right => dir = Direction::Right,
                    KeyCode::Char('q') => break, // Nhấn Q để thoát
                    _ => {}
                }
            }
        }

        // 2. Cập nhật logic sau mỗi 150ms
        if last_tick.elapsed() >= Duration::from_millis(500) {
            let head = snake[0];
            let mut new_head = match dir {
                Direction::Up => Point {
                    x: head.x,
                    y: head.y.saturating_sub(1),
                },
                Direction::Down => Point {
                    x: head.x,
                    y: head.y + 1,
                },
                Direction::Left => Point {
                    x: head.x.saturating_sub(1),
                    y: head.y,
                },
                Direction::Right => Point {
                    x: head.x + 1,
                    y: head.y,
                },
            };

            // Kiểm tra va chạm mồi
            if new_head == food {
                food = Point {
                    x: rand_num(1, 20),
                    y: rand_num(1, 10),
                }; // Tạo mồi mới
            } else {
                snake.pop(); // Không ăn mồi thì xóa đuôi
            }

            // Kiểm tra va chạm thân hoặc biên (đơn giản)
            if snake.contains(&new_head) || new_head.x > 40 || new_head.y > 20 {
                break; // Game Over
            }

            snake.insert(0, new_head);
            last_tick = Instant::now();
        }

        // 3. Vẽ lên Terminal
        execute!(stdout, terminal::Clear(ClearType::All))?;
        for p in &snake {
            execute!(stdout, cursor::MoveTo(p.x, p.y))?;
            print!("■");
        }
        execute!(stdout, cursor::MoveTo(food.x, food.y))?;
        print!("★");
        stdout.flush()?;
    }

    // Khôi phục terminal
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    println!("\nGame Over!");
    Ok(())
}

fn rand_num(min: u16, max: u16) -> u16 {
    // Hàm tạo số giả ngẫu nhiên đơn giản không cần dùng crate rand
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
        % (max - min) as u128) as u16
        + min
}
